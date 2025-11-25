use bevy_ecs::{
	component::{Component, HookContext},
	entity::{ContainsEntity, Entity},
	error::BevyError,
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use disqualified::ShortName;
use rx_core_macro_subject_derive::RxSubject;
use rx_core_traits::{
	Observer as RxObserver, ObserverNotification, ObserverPushObserverNotificationExtention,
	SubjectLike, SubscriptionLike,
};
use stealcell::{StealCell, Stolen};

use crate::{
	DeferredWorldAsRxBevyContextExtension, ObservableSubscriptions, RxBevyContext,
	RxBevyContextItem, RxSignal, ScheduledSubscriptionComponent, Subscribe, SubscribeError,
	SubscribeObserverOf, SubscribeObserverRef, SubscribeObserverTypeMarker, SubscriptionOf,
	UnfinishedSubscription, default_on_subscribe_error_handler,
};

#[derive(Component, RxSubject)]
#[component(on_insert=subject_on_insert::<Subject>, on_remove=subject_on_remove::<Subject>)]
#[require(ObservableSubscriptions::<Subject>)]
#[rx_in(Subject::In)]
#[rx_in_error(Subject::InError)]
#[rx_out(Subject::Out)]
#[rx_out_error(Subject::OutError)]
#[rx_context(RxBevyContext)]
pub struct SubjectComponent<Subject>
where
	Subject: SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	subject: StealCell<Subject>,
}

impl<Subject> SubjectComponent<Subject>
where
	Subject: SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	pub fn new(subject: Subject) -> Self {
		Self {
			subject: StealCell::new(subject),
		}
	}

	pub(crate) fn steal_subject(&mut self) -> Stolen<Subject> {
		self.subject.steal()
	}

	pub(crate) fn return_stolen_subject(&mut self, observable: Stolen<Subject>) {
		self.subject.return_stolen(observable);
	}
}

// TODO: This is actually not used, delete if no usecase is found together with the derive
impl<Subject> RxObserver for SubjectComponent<Subject>
where
	Subject: SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	fn next(
		&mut self,
		next: Self::In,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.get_mut().next(next, context);
	}

	fn error(
		&mut self,
		error: Self::InError,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.get_mut().error(error, context);
	}

	fn complete(
		&mut self,
		context: &mut <Self::Context as rx_core_traits::SubscriptionContext>::Item<'_, '_>,
	) {
		self.subject.get_mut().complete(context);
	}
}

fn subject_on_insert<Subject>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<Subject, bevy_app::Update, bevy_time::Virtual>(
		&mut deferred_world,
	);

	let mut commands = deferred_world.commands();
	let _subscribe_event_observer_id = commands
		.spawn((
			// TODO(bevy-0.17): This is actually not needed, it's only here to not let these observes occupy the top level in the worldentityinspector. reconsider to only use either this or the other relationship if it's still producing warnings on despawn in 0.17
			ChildOf(hook_context.entity),
			SubscribeObserverOf::<Subject>::new(hook_context.entity),
			SubscribeObserverTypeMarker::<Subject::Out, Subject::OutError>::default(),
			Name::new(format!("Subscribe Observer {}", ShortName::of::<Subject>())),
			Observer::new(subscribe_event_observer::<Subject>)
				.with_entity(hook_context.entity)
				.with_error_handler(default_on_subscribe_error_handler),
		))
		.id();

	commands.spawn((
		ChildOf(hook_context.entity),
		Name::new(format!(
			"Notification Observer {}",
			ShortName::of::<Subject>()
		)),
		Observer::new(subject_notification_observer::<Subject>).with_entity(hook_context.entity),
	));
}

fn subject_notification_observer<'w, 's, Subject>(
	on_notification: Trigger<RxSignal<Subject::In, Subject::InError>>,
	mut context: RxBevyContextItem<'w, 's>,
) where
	Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	let subject_entity = on_notification.entity();

	let notification: ObserverNotification<Subject::In, Subject::InError> =
		on_notification.event().clone().into();

	let mut stolen_subject = context.steal_subject::<Subject>(subject_entity).unwrap();

	stolen_subject.push(
		notification,
		&mut context, // I have to access the context, passing it into something that was accessed from the context
	);
	context
		.return_stolen_subject(subject_entity, stolen_subject)
		.unwrap();
}

fn subscribe_event_observer<'w, 's, Subject>(
	mut on_subscribe: Trigger<Subscribe<Subject::Out, Subject::OutError>>,
	mut context: RxBevyContextItem<'w, 's>,
) -> Result<(), BevyError>
where
	Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	let event = on_subscribe.event_mut();

	let Some(destination) = event.try_consume_destination() else {
		return Err(SubscribeError::EventAlreadyConsumed(
			ShortName::of::<Subject>().to_string(),
			event.observable_entity,
		)
		.into());
	};

	let subscription = {
		let mut stolen_subject = context.steal_subject::<Subject>(event.observable_entity)?;
		let subscription = stolen_subject.subscribe(
			destination,
			&mut context, // I have to access the context, passing it into something that was accessed from the context
		);
		context.return_stolen_subject(event.observable_entity, stolen_subject)?;
		subscription
	};

	let mut commands = context.deferred_world.commands();
	let mut subscription_entity_commands = commands.entity(event.subscription_entity);

	if !subscription.is_closed() {
		// Instead of spawning a new entity here, a pre-spawned one is used that the user
		// already has access to.
		// It also already contains the [SubscriptionSchedule] component.
		subscription_entity_commands.insert((
			ScheduledSubscriptionComponent::new(subscription, event.subscription_entity),
			SubscriptionOf::<Subject>::new(event.observable_entity),
		));
	} else {
		subscription_entity_commands.try_despawn();
	}

	// Marks the subscription entity as "finished".
	// An "unfinished" subscription entity would be immediately despawned.
	subscription_entity_commands.try_remove::<UnfinishedSubscription>();

	Ok(())
}

/// Remove related components along with the observable
fn subject_on_remove<Subject>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<Subject>>()
		.remove::<SubscribeObserverRef<Subject>>();

	let mut context = deferred_world.into_rx_context();

	let mut stolen_subject = context
		.steal_subject::<Subject>(hook_context.entity)
		.unwrap();
	stolen_subject.unsubscribe(&mut context);
	context
		.return_stolen_subject(hook_context.entity, stolen_subject)
		.unwrap();
}

pub trait BevyContextSubjectStealingExt {
	fn steal_subject<Subject>(&mut self, entity: Entity) -> Result<Stolen<Subject>, BevyError>
	where
		Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
		Subject::In: Clone,
		Subject::InError: Clone;

	fn return_stolen_subject<Subject>(
		&mut self,
		entity: Entity,
		subject: Stolen<Subject>,
	) -> Result<(), BevyError>
	where
		Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
		Subject::In: Clone,
		Subject::InError: Clone;
}

impl<'w, 's> BevyContextSubjectStealingExt for RxBevyContextItem<'w, 's> {
	fn steal_subject<Subject>(&mut self, entity: Entity) -> Result<Stolen<Subject>, BevyError>
	where
		Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
		Subject::In: Clone,
		Subject::InError: Clone,
	{
		let mut subject_component =
			self.try_get_component_mut::<SubjectComponent<Subject>>(entity)?;
		Ok(subject_component.steal_subject())
	}

	fn return_stolen_subject<Subject>(
		&mut self,
		entity: Entity,
		subject: Stolen<Subject>,
	) -> Result<(), BevyError>
	where
		Subject: 'static + SubjectLike<Context = RxBevyContext> + Send + Sync,
		Subject::In: Clone,
		Subject::InError: Clone,
	{
		let mut subject_component =
			self.try_get_component_mut::<SubjectComponent<Subject>>(entity)?;

		subject_component.return_stolen_subject(subject);

		Ok(())
	}
}
