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

	/// Clones an existing subscription and updates it's source and destination entities.
	/// Useful for preserving its scheduling without knowing what that schedule was.
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn clone_and_retarget_subscription<Out, OutError, NextOut, NextOutError>(
		&mut self,
		subscription_event: &Subscribe<Out, OutError>,
		new_observable_entity: Entity,
		new_subscriber_entity: Entity,
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

	/// TODO: Deprecate, can't work on the same frame, maybe it will in a later bevy version
	fn clone_and_retarget_subscription<Out, OutError, NewOut, NewOutError>(
		&mut self,
		subscribe_event: &Subscribe<Out, OutError>,
		new_observable_entity: Entity,
		new_subscriber_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		NewOut: SignalBound,
		NewOutError: SignalBound,
	{
		let (event, subscription_entity) =
			subscribe_event.retarget_existing::<NewOut, NewOutError>(new_subscriber_entity, self);
		dbg!(event.clone());
		dbg!(new_observable_entity);
		self.trigger_targets(event, new_observable_entity);

		subscription_entity
	}
}
