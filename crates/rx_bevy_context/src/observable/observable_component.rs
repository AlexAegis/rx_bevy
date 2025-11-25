use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::{BevyError, ErrorContext},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	world::DeferredWorld,
};
use bevy_log::error;
use disqualified::ShortName;
use rx_core_traits::{Observable, SubscriptionLike};
use stealcell::{StealCell, Stolen};
use thiserror::Error;

use crate::{
	BevySubscriptionContext, BevySubscriptionContextParam, BevySubscriptionContextProvider,
	ObservableOutputs, ObservableSubscriptions, ScheduledSubscriptionComponent, Subscribe,
	SubscribeObserverOf, SubscribeObserverRef, SubscribeObserverTypeMarker, SubscriptionOf,
	UnfinishedSubscription,
};

#[derive(Component)]
#[component(on_insert=observable_on_insert::<O>, on_remove=observable_on_remove::<O>)]
#[require(ObservableSubscriptions::<O>, ObservableOutputs::<O::Out, O::OutError>)]
pub struct ObservableComponent<O>
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	observable: StealCell<O>,
}

impl<O> ObservableComponent<O>
where
	O: Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	pub fn new(observable: O) -> Self {
		Self {
			observable: StealCell::new(observable),
		}
	}

	pub(crate) fn steal_observable(&mut self) -> Stolen<O> {
		self.observable.steal()
	}

	pub(crate) fn return_stolen_observable(&mut self, observable: Stolen<O>) {
		self.observable.return_stolen(observable);
	}
}

fn observable_on_insert<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<O, bevy_app::Update, bevy_time::Virtual>(
		&mut deferred_world,
	);

	let _subscribe_event_observer_id = deferred_world
		.commands()
		.spawn((
			// TODO(bevy-0.17): This is actually not needed, it's only here to not let these observes occupy the top level in the worldentityinspector. reconsider to only use either this or the other relationship if it's still producing warnings on despawn in 0.17
			ChildOf(hook_context.entity),
			SubscribeObserverOf::<O>::new(hook_context.entity),
			SubscribeObserverTypeMarker::<O::Out, O::OutError>::default(),
			Name::new(format!(
				"Subscribe Observer <Out = {}, OutError = {}> ({})",
				ShortName::of::<O::Out>(),
				ShortName::of::<O::OutError>(),
				ShortName::of::<O>()
			)),
			Observer::new(subscribe_event_observer::<O>)
				.with_entity(hook_context.entity)
				.with_error_handler(default_on_subscribe_error_handler),
		))
		.id();
}

fn subscribe_event_observer<'w, 's, O>(
	mut on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	context_param: BevySubscriptionContextParam<'w, 's>,
) -> Result<(), BevyError>
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	let event = on_subscribe.event_mut();

	let Some(destination) = event.try_consume_destination() else {
		return Err(SubscribeError::EventAlreadyConsumed(
			ShortName::of::<O>().to_string(),
			event.observable_entity,
		)
		.into());
	};

	let mut context = context_param.into_context(Some(event.subscription_entity));

	let subscription = {
		let mut stolen_observable = context.steal_observable::<O>(event.observable_entity)?;
		let subscription = stolen_observable.subscribe(
			destination,
			&mut context, // I have to access the context, passing it into something that was accessed from the context
		);
		context.return_stolen_observable(event.observable_entity, stolen_observable)?;
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
			SubscriptionOf::<O>::new(event.observable_entity),
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
fn observable_on_remove<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
{
	deferred_world
		.commands()
		.entity(hook_context.entity)
		.remove::<ObservableSubscriptions<O>>()
		.remove::<SubscribeObserverRef<O>>();
}

/// Errors that can happen during a [Subscribe] event.
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
	#[error(
		"Tried to subscribe to {0} on {1}. But the Subscribe event already had it's destination consumed!"
	)]
	EventAlreadyConsumed(String, Entity),
	// TODO: consider how this could be implemented now, or if it's even needed. self subscriptions on subjects would cause infinite loops, maybe subjects could be treated as special things and have their own components which could be used to to trigger this error with an associuated const
	// #[error("Tried to subscribe to {0}. But it disallows subscriptions from the same entity {1}.")]
	// SelfSubscribeDisallowed(String, Entity),
}

/// The default error handler just prints out the error as warning
pub(crate) fn default_on_subscribe_error_handler(error: BevyError, error_context: ErrorContext) {
	if let Some(subscribe_error) = error.downcast_ref::<SubscribeError>() {
		error!("{}", subscribe_error);
	} else {
		panic!(
			"Unknown error happened during subscribe. Kind: {}\tName: {}",
			error_context.kind(),
			error_context.name()
		);
	}
}

pub trait BevyContextObservableStealingExt {
	fn steal_observable<O>(&mut self, entity: Entity) -> Result<Stolen<O>, BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync;

	fn return_stolen_observable<O>(
		&mut self,
		entity: Entity,
		observable: Stolen<O>,
	) -> Result<(), BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync;
}

impl<'w, 's> BevyContextObservableStealingExt for BevySubscriptionContext<'w, 's> {
	fn steal_observable<O>(&mut self, entity: Entity) -> Result<Stolen<O>, BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
	{
		let mut obserable_component =
			self.try_get_component_mut::<ObservableComponent<O>>(entity)?;
		Ok(obserable_component.steal_observable())
	}

	fn return_stolen_observable<O>(
		&mut self,
		entity: Entity,
		observable: Stolen<O>,
	) -> Result<(), BevyError>
	where
		O: 'static + Observable<Context = BevySubscriptionContextProvider> + Send + Sync,
	{
		let mut obserable_component =
			self.try_get_component_mut::<ObservableComponent<O>>(entity)?;

		obserable_component.return_stolen_observable(observable);

		Ok(())
	}
}
