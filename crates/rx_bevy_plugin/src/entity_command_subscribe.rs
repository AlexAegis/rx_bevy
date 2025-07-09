use bevy_ecs::{
	entity::Entity,
	schedule::ScheduleLabel,
	system::{Commands, EntityCommands},
};

use crate::{ObservableComponent, ObservableSignalBound, RelativeEntity, Subscribe};

/// TODO: Right now this is just a despawn alias, but it should be possible to unsubscribe from Subjects too, so this should be an Event! But the already contain observers? A command maybe?
pub trait CommandsUnsubscribeExtension {
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandsUnsubscribeExtension for Commands<'w, 's> {
	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).despawn();
	}
}

pub trait EntityCommandSubscribeExtension {
	/// Subscribes the observable on THIS entity, to an observer entity
	/// Returns the entity of the subscription which you can despawn to unsubscribe it
	/// TODO: Instead of O, it should be the Signal, OR create a single ObservableComponent that can house any kind of observables
	#[must_use]
	fn subscribe_to_this_scheduled<O, S>(&mut self, subscriber_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel;

	#[must_use]
	fn subscribe_to_this_unscheduled<O>(&mut self, subscriber_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound;

	#[must_use]
	fn subscribe_to_that_scheduled<O, S>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel;

	#[must_use]
	fn subscribe_to_that_unscheduled<O>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound;
}

impl<'a> EntityCommandSubscribeExtension for EntityCommands<'a> {
	fn subscribe_to_this_scheduled<O, S>(&mut self, subscriber_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) =
			Subscribe::<O>::scheduled::<S>(subscriber_entity, commands);

		commands.trigger_targets(event, observable_entity);

		subscription_entity
	}

	fn subscribe_to_this_unscheduled<O>(&mut self, subscriber_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
	{
		let observable_entity = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) = Subscribe::<O>::unscheduled(subscriber_entity, commands);

		commands.trigger_targets(event, observable_entity);

		subscription_entity
	}

	fn subscribe_to_that_scheduled<O, S>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
		S: ScheduleLabel,
	{
		let subscriber_entity = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) =
			Subscribe::<O>::scheduled::<S>(RelativeEntity::Other(subscriber_entity), commands);

		commands.trigger_targets(event, observable_entity.this_or(subscriber_entity));

		subscription_entity
	}

	fn subscribe_to_that_unscheduled<O>(&mut self, observable_entity: RelativeEntity) -> Entity
	where
		O: ObservableComponent,
		O::Out: ObservableSignalBound,
		O::OutError: ObservableSignalBound,
	{
		let subscriber_entity = self.id();
		let commands = self.commands_mut();
		let (event, subscription_entity) =
			Subscribe::<O>::unscheduled(RelativeEntity::Other(subscriber_entity), commands);

		commands.trigger_targets(event, observable_entity.this_or(subscriber_entity));

		subscription_entity
	}
}
