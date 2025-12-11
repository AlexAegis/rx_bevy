use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	error::{BevyError, ErrorContext},
	hierarchy::ChildOf,
	name::Name,
	observer::{Observer, Trigger},
	system::{Commands, Query, ResMut},
	world::DeferredWorld,
};
use bevy_log::error;
use disqualified::ShortName;
use rx_core_macro_observable_derive::RxObservable;
use rx_core_traits::{Observable, SubscriptionLike, TaskExecutor};
use thiserror::Error;

use crate::{
	ObservableOutputs, ObservableSubscriptions, RxBevyExecutorLast, Subscribe, SubscribeObserverOf,
	SubscribeObserverRef, SubscribeObserverTypeMarker, SubscriptionComponent, SubscriptionOf,
	UnfinishedSubscription,
};

#[derive(Component, RxObservable)]
#[rx_out(O::Out)]
#[rx_out_error(O::OutError)]
#[component(on_insert=observable_on_insert::<O>, on_remove=observable_on_remove::<O>)]
#[require(ObservableSubscriptions::<O>, ObservableOutputs::<O::Out, O::OutError>)]
pub struct ObservableComponent<O>
where
	O: Observable + Send + Sync,
{
	observable: O,
}

impl<O> ObservableComponent<O>
where
	O: Observable + Send + Sync,
{
	pub fn new(observable: O) -> Self {
		Self { observable }
	}
}

impl<O> Observable for ObservableComponent<O>
where
	O: Observable + Send + Sync,
{
	type Subscription<Destination>
		= O::Subscription<Destination>
	where
		Destination: 'static + rx_core_traits::Subscriber<In = Self::Out, InError = Self::OutError>;

	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ rx_core_traits::UpgradeableObserver<In = Self::Out, InError = Self::OutError>
			+ Send
			+ Sync,
	{
		self.observable.subscribe(destination)
	}
}

fn observable_on_insert<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable + Send + Sync,
{
	#[cfg(feature = "debug")]
	crate::register_observable_debug_systems::<O>(&mut deferred_world);

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

fn subscribe_event_observer<O>(
	mut on_subscribe: Trigger<Subscribe<O::Out, O::OutError>>,
	mut commands: Commands,
	mut observable_query: Query<&mut ObservableComponent<O>>,
	last_executor: ResMut<RxBevyExecutorLast>,
) -> Result<(), BevyError>
where
	O: 'static + Observable + Send + Sync,
{
	let event = on_subscribe.event_mut();

	let Some(destination) = event.try_consume_destination() else {
		return Err(SubscribeError::EventAlreadyConsumed(
			ShortName::of::<O>().to_string(),
			event.observable_entity,
		)
		.into());
	};

	let subscription = {
		let mut observable_component = observable_query.get_mut(event.observable_entity).unwrap();

		observable_component.subscribe(destination)
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
				last_executor.get_scheduler_handle(),
			),
			SubscriptionOf::<O>::new(event.observable_entity),
		));
	} else {
		// The subscription is already closed, despawn the pre-spawned subscription entity
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
	O: 'static + Observable + Send + Sync,
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
