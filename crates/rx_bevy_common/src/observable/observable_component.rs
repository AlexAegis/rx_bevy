use bevy_ecs::{
	component::{Component, HookContext},
	entity::Entity,
	world::DeferredWorld,
};
use rx_core_common::Observable;
use rx_core_macro_observable_derive::RxObservable;
use thiserror::Error;

use crate::{
	ObservableOutputs, ObservableSubscriptions, SubscribeEventObserverSatelliteBundle,
	SubscribeObserverRef,
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
		Destination: 'static + rx_core_common::Subscriber<In = Self::Out, InError = Self::OutError>;

	#[inline]
	fn subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscription<Destination::Upgraded>
	where
		Destination: 'static
			+ rx_core_common::UpgradeableObserver<In = Self::Out, InError = Self::OutError>
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
	deferred_world
		.commands()
		.spawn(SubscribeEventObserverSatelliteBundle::<O>::new::<
			ObservableComponent<O>,
		>(hook_context.entity));
}

/// Remove related components along with the observable
fn observable_on_remove<O>(mut deferred_world: DeferredWorld, hook_context: HookContext)
where
	O: 'static + Observable + Send + Sync,
{
	let subscribe_observer_ref = deferred_world
		.get::<SubscribeObserverRef<O>>(hook_context.entity)
		.map(|observer_ref| **observer_ref);

	let mut commands = deferred_world.commands();

	if let Some(subscribe_observer_entity) = subscribe_observer_ref {
		commands.entity(subscribe_observer_entity).try_despawn();
	}

	commands
		.entity(hook_context.entity)
		.try_remove::<ObservableSubscriptions<O>>()
		.try_remove::<SubscribeObserverRef<O>>()
		.try_remove::<ObservableOutputs<O::Out, O::OutError>>();
}

/// Errors that can happen during a [Subscribe] event.
#[derive(Error, Debug)]
pub enum SubscribeError {
	#[error("Tried to subscribe to {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
	#[error(
		"Tried to subscribe to {0} on {1}. But the Subscribe event already had its destination consumed!"
	)]
	EventAlreadyConsumed(String, Entity),
}
