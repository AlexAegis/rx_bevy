use bevy_ecs::{
	entity::Entity,
	schedule::ScheduleLabel,
	system::{Commands, EntityCommands},
};
use rx_bevy_common_bounds::SignalBound;

use crate::{CommandSubscribeExtension, RelativeEntity};

/// TODO: Right now this is just a despawn alias, but it should be possible to unsubscribe from Subjects too, so this should be an Event! But the already contain observers? A command maybe?
pub trait CommandsUnsubscribeExtension {
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandsUnsubscribeExtension for Commands<'w, 's> {
	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).despawn();
	}
}

/// Provides commands for subscription relative to this entity
pub trait EntityCommandSubscribeExtension {
	/// Subscribes the observable on THIS entity, to an observer entity
	/// Returns the entity of the subscription which you can despawn to unsubscribe it
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_to_this_scheduled<Out, OutError, S>(
		&mut self,
		subscriber_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_to_this_unscheduled<Out, OutError>(
		&mut self,
		subscriber_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_to_that_scheduled<Out, OutError, S>(
		&mut self,
		observable_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_to_that_unscheduled<Out, OutError>(
		&mut self,
		observable_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe_to_this_scheduled<Out, OutError, S>(
		&mut self,
		subscriber_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let observable_entity = self.id();
		let subscriber_entity = subscriber_entity.or_this(observable_entity);
		let commands = self.commands_mut();
		commands.subscribe_scheduled::<Out, OutError, S>(observable_entity, subscriber_entity)
	}

	fn subscribe_to_this_unscheduled<Out, OutError>(
		&mut self,
		subscriber_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
	{
		let observable_entity = self.id();
		let subscriber_entity = subscriber_entity.or_this(observable_entity);
		let commands = self.commands_mut();
		commands.subscribe_unscheduled::<Out, OutError>(observable_entity, subscriber_entity)
	}

	fn subscribe_to_that_scheduled<Out, OutError, S>(
		&mut self,
		observable_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let subscriber_entity = self.id();
		let observable_entity = observable_entity.or_this(subscriber_entity);
		let commands = self.commands_mut();
		commands.subscribe_scheduled::<Out, OutError, S>(observable_entity, subscriber_entity)
	}

	fn subscribe_to_that_unscheduled<Out, OutError>(
		&mut self,
		observable_entity: RelativeEntity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
	{
		let subscriber_entity = self.id();
		let observable_entity = observable_entity.or_this(subscriber_entity);
		let commands = self.commands_mut();
		commands.subscribe_unscheduled::<Out, OutError>(observable_entity, subscriber_entity)
	}
}
