use bevy_ecs::{
	component::{Component, HookContext},
	entity::ContainsEntity,
	error::BevyError,
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query},
	world::DeferredWorld,
};
use disqualified::ShortName;
use rx_core_common::{
	Observable, Observer as RxObserver, ObserverNotification,
	ObserverPushObserverNotificationExtention, SubjectLike, Subscriber, SubscriptionLike,
	UpgradeableObserver,
};
use rx_core_macro_subject_derive::RxSubject;

use crate::{
	ObservableSubscriptions, RxScheduleDespawn, RxSignal, Subscribe, SubscribeError,
	SubscribeObserverOf, SubscribeObserverRef, SubscribeObserverTypeMarker, SubscriptionComponent,
	SubscriptionOf, UnfinishedSubscription, default_on_subscribe_error_handler,
};

/// Note that if you accidentally subscribe to a subject entity with itself,
/// then that will result in an infinite loop! With a regular Subject it's
/// easy and self evident that this would happen, but since it's possible
/// to subscribe to stuff by only knowing it's output types, it's harder to
/// know when is it a subject or just an observable. Although it should be
/// rare that even a regular observable would send events to the same entity
/// it's defined on.
#[derive(Component, RxSubject)]
#[rx_in(Subject::In)]
#[rx_in_error(Subject::InError)]
#[rx_out(Subject::Out)]
#[rx_out_error(Subject::OutError)]
#[rx_delegate_subscription_like_to_destination]
#[component(on_insert=subject_on_insert::<Subject>, on_remove=subject_on_remove::<Subject>)]
#[require(ObservableSubscriptions::<Subject>)]
pub struct SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[destination]
	subject: Subject,
}

impl<Subject> SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	pub fn new(subject: Subject) -> Self {
		Self { subject }
	}
}

impl<Subject> Observable for SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	type Subscription<Destination>
		= Subject::Subscription<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination:
			'static + UpgradeableObserver<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		self.subject.subscribe(destination)
	}
}

impl<Subject> RxObserver for SubjectComponent<Subject>
where
	Subject: SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[inline]
	fn next(&mut self, next: Self::In) {
		self.subject.next(next);
	}

	#[inline]
	fn error(&mut self, error: Self::InError) {
		self.subject.error(error);
	}

	#[inline]
	fn complete(&mut self) {
		self.subject.complete();
	}
}

fn subject_on_insert<Subject>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	Subject: 'static + SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<Subject>(&mut deferred_world);

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

fn subject_notification_observer<Subject>(
	on_notification: Trigger<RxSignal<Subject::In, Subject::InError>>,
	mut subject_query: Query<&mut SubjectComponent<Subject>>,
) where
	Subject: 'static + SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	let subject_entity = on_notification.entity();
	if let Ok(mut subject) = subject_query.get_mut(subject_entity) {
		let notification: ObserverNotification<Subject::In, Subject::InError> =
			on_notification.event().clone().into();
		subject.push(notification);
	}
}

fn subscribe_event_observer<Subject>(
	mut on_subscribe: Trigger<Subscribe<Subject::Out, Subject::OutError>>,
	mut subject_query: Query<&mut SubjectComponent<Subject>>,
	mut commands: Commands,
	rx_schedule_despawn: RxScheduleDespawn,
) -> Result<(), BevyError>
where
	Subject: 'static + SubjectLike + Send + Sync,
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
		let mut subject_component = subject_query.get_mut(event.observable_entity).unwrap();
		subject_component.subscribe(destination)
	};

	let mut subscription_entity_commands = commands.entity(event.subscription_entity);

	if !subscription.is_closed() {
		// Instead of spawning a new entity here, a pre-spawned one is used that the user
		// already has access to.
		// It also already contains the [SubscriptionSchedule] component.
		subscription_entity_commands.insert((
			SubscriptionComponent::new(
				subscription,
				event.subscription_entity,
				rx_schedule_despawn.handle(),
			),
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
	Subject: 'static + SubjectLike + Send + Sync,
	Subject::In: Clone,
	Subject::InError: Clone,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<Subject>>()
		.remove::<SubscribeObserverRef<Subject>>();

	let mut subject_component = deferred_world
		.get_mut::<SubjectComponent<Subject>>(hook_context.entity)
		.unwrap();

	subject_component.unsubscribe();
}
