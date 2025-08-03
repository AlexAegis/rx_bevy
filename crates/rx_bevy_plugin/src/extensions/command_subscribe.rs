use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::Commands};

use crate::{RelativeEntity, SignalBound, Subscribe};

/// Provides functions to create subscriptions between two commands
pub trait CommandSubscribeExtension {
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_scheduled<Out, OutError, S>(
		&mut self,
		observable_entity: Entity,
		subscriber_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_unscheduled<Out, OutError>(
		&mut self,
		observable_entity: Entity,
		subscriber_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound;

	/// Subscribes using the same schedule as an existing Subscribe event.
	/// Subscribe events contain erased information about their schedule, which
	/// is used to create a chain of subscriptions with the same schedule.
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_with_schedule_of<Out, OutError, NextOut, NextOutError>(
		&mut self,
		observable_entity: Entity,
		subscriber_entity: Entity,
		subscription_event: &Subscribe<Out, OutError>,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		NextOut: SignalBound,
		NextOutError: SignalBound;
}

impl<'w, 's> CommandSubscribeExtension for Commands<'w, 's> {
	fn subscribe_scheduled<Out, OutError, S>(
		&mut self,
		observable_entity: Entity,
		subscriber_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let (event, subscription_entity) = Subscribe::<Out, OutError>::scheduled::<S>(
			RelativeEntity::Other(subscriber_entity),
			self,
		);

		self.trigger_targets(event, observable_entity);

		subscription_entity
	}

	fn subscribe_unscheduled<Out, OutError>(
		&mut self,
		observable_entity: Entity,
		subscriber_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
	{
		let (event, subscription_entity) =
			Subscribe::<Out, OutError>::unscheduled(RelativeEntity::Other(subscriber_entity), self);

		self.trigger_targets(event, observable_entity);

		subscription_entity
	}

	fn subscribe_with_schedule_of<Out, OutError, NewOut, NewOutError>(
		&mut self,
		new_observable_entity: Entity,
		new_subscriber_entity: Entity,
		use_schedule_from: &Subscribe<Out, OutError>,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		NewOut: SignalBound,
		NewOutError: SignalBound,
	{
		let (event, subscription_entity) = Subscribe::<NewOut, NewOutError>::new_with_schedule_from(
			new_subscriber_entity,
			use_schedule_from,
			self,
		);
		self.trigger_targets(event, new_observable_entity);

		subscription_entity
	}
}
