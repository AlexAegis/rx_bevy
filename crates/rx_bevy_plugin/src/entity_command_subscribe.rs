use bevy_ecs::{
	entity::Entity,
	schedule::ScheduleLabel,
	system::{Commands, EntityCommands},
};

use crate::{ObservableComponent, ObservableSignalBound, SubscribeFor, SubscriberEntity};

pub trait CommandsUnsubscribeExtension {
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandsUnsubscribeExtension for Commands<'w, 's> {
	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).despawn();
	}
}

/// TODO: Add 2 more subscribe_to_that_scheduled/unscheduled
pub trait EntityCommandSubscribeExtension {
	/// Subscribes the observable on THIS entity, to an observer entity
	/// Returns the entity of the subscription which you can despawn to unsubscribe it
	/// TODO: Instead of O, it should be the Signal, OR create a single ObservableComponent that can house any kind of observables
	#[must_use]
	fn subscribe_to_this_scheduled<O, S>(&mut self, subscriber_entity: SubscriberEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel;

	#[must_use]
	fn subscribe_to_this_unscheduled<O>(&mut self, subscriber_entity: SubscriberEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe_to_this_scheduled<O, S>(&mut self, subscriber_entity: SubscriberEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel,
	{
		let self_id = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) =
			SubscribeFor::<O>::scheduled::<S>(subscriber_entity, commands);

		commands.trigger_targets(event, self_id);

		subscription_entity
	}

	fn subscribe_to_this_unscheduled<O>(&mut self, subscriber_entity: SubscriberEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
	{
		let self_id = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) =
			SubscribeFor::<O>::unscheduled(subscriber_entity, commands);

		commands.trigger_targets(event, self_id);

		subscription_entity
	}
}
