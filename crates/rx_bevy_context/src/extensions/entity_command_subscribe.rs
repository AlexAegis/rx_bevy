use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::EntityCommands};
use rx_bevy_common::Clock;
use rx_core_traits::{SignalBound, UpgradeableObserver};

use crate::{BevySubscriptionContextProvider, CommandSubscribeExtension, EntityDestination};

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
	fn subscribe_destination<Destination, S, C>(&mut self, destination: Destination) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
		S: ScheduleLabel,
		C: Clock;

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
	fn subscribe<Out, OutError, S, C>(&mut self, destination_entity: Entity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
		C: Clock;

	/// # subscribes_to_observable_entity
	///
	/// Subscribes to the observable on the entity passed in, with this entity
	/// as the destination entity. This entity will receive the notifications.
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribes_to_observable_entity<Out, OutError, S, C>(
		&mut self,
		observable_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
		C: Clock;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe_destination<Destination, S, C>(&mut self, destination: Destination) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
		S: ScheduleLabel,
		C: Clock,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();
		commands.subscribe::<_, S, C>(observable_entity, destination)
	}

	fn subscribe<Out, OutError, S, C>(&mut self, destination_entity: Entity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
		C: Clock,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();
		commands.subscribe::<_, S, C>(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity),
		)
	}

	fn subscribes_to_observable_entity<Out, OutError, S, C>(
		&mut self,
		observable_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
		C: Clock,
	{
		let destination_entity = self.id();
		let commands = self.commands_mut();
		commands.subscribe::<_, S, C>(
			observable_entity,
			EntityDestination::<Out, OutError>::new(destination_entity),
		)
	}
}
