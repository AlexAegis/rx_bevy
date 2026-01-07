use bevy_ecs::{entity::Entity, system::EntityCommands};
use rx_core_common::{SchedulerHandle, Signal, UpgradeableObserver};

use crate::{CommandSubscribeExtension, EntityDestination, RxBevyScheduler};

/// Provides commands for subscription relative to this entity
pub trait EntityCommandSubscribeExtension {
	/// # subscribe_with
	///
	/// Subscribes to an observable on THIS entity, with a destination
	/// subscriber.
	///
	/// Returns the entity of the subscription which you can despawn to
	/// unsubscribe it.
	///
	/// The subscription will only be successful if an observable component
	/// exists on this entity with the same output types as the destinations
	/// input types.
	///
	/// ## Warning for entities of multiple matching Observables
	///
	/// If multiple matching observable components exist, only one of them
	/// will be subscribed to as the first one found will consume the
	/// destination on the event!
	///
	///
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_destination<Destination>(&mut self, destination: Destination) -> Entity
	where
		Destination: 'static + UpgradeableObserver;

	/// # subscribe
	///
	/// Subscribes to an observable on THIS entity, with a destination entity
	/// that will receive [RxSignal] events!
	///
	/// The subscription will only be successful if an observable component
	/// exists on this entity with the same output types as specified on the
	/// function call.
	///
	/// ## Warning for entities of multiple matching Observables
	///
	/// If multiple matching observable components exist, only one of them
	/// will be subscribed to as the first one found will consume the
	/// destination on the event!
	///
	/// Returns the entity of the subscription which you can despawn to unsubscribe it
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Out, OutError>(
		&mut self,
		destination_entity: Entity,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Entity
	where
		Out: Signal,
		OutError: Signal;

	/// # subscribes_to_observable_entity
	///
	/// Subscribes to the observable on the entity passed in, with this entity
	/// as the destination entity. This entity will receive the notifications.
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribes_to_observable_entity<Out, OutError>(
		&mut self,
		observable_entity: Entity,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Entity
	where
		Out: Signal,
		OutError: Signal;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe_destination<Destination>(&mut self, destination: Destination) -> Entity
	where
		Destination: 'static + UpgradeableObserver,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();
		commands.subscribe(observable_entity, destination)
	}

	fn subscribe<Out, OutError>(
		&mut self,
		destination_entity: Entity,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Entity
	where
		Out: Signal,
		OutError: Signal,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();

		commands.subscribe(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity, scheduler),
		)
	}

	fn subscribes_to_observable_entity<Out, OutError>(
		&mut self,
		observable_entity: Entity,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Entity
	where
		Out: Signal,
		OutError: Signal,
	{
		let destination_entity = self.id();
		let commands = self.commands_mut();
		commands.subscribe(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity, scheduler),
		)
	}
}
