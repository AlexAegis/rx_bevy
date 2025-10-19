use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::EntityCommands};
use rx_core_traits::SignalBound;

use crate::{CommandSubscribeExtension, RelativeEntity};

/// Provides commands for subscription relative to this entity
pub trait EntityCommandSubscribeExtension {
	/// Subscribes to the observable on THIS entity, with a destination entity
	/// that will receive notifications.
	///
	/// Returns the entity of the subscription which you can despawn to unsubscribe it
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Out, OutError, S>(&mut self, destination_entity: RelativeEntity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;

	/// Subscribes to the observable on the entity passed in, with this entity
	/// as the destination entity. This entity will receive the notifications.
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribes_to<Out, OutError, S>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe<Out, OutError, S>(&mut self, destination_entity: RelativeEntity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let observable_entity = self.id();
		let destination_entity = destination_entity.or_this(observable_entity);
		let commands = self.commands_mut();
		commands.subscribe::<Out, OutError, S>(observable_entity, destination_entity)
	}

	fn subscribes_to<Out, OutError, S>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let destination_entity = self.id();
		let observable_entity = observable_entity.or_this(destination_entity);
		let commands = self.commands_mut();
		commands.subscribe::<Out, OutError, S>(observable_entity, destination_entity)
	}
}
