use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::Commands};
use rx_core_traits::SignalBound;

use crate::Subscribe;

pub trait CommandsUnsubscribeExtension {}

/// Provides functions to create subscriptions between two commands
pub trait CommandSubscribeExtension {
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Out, OutError, S>(
		&mut self,
		observable_entity: Entity,
		destination_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel;

	/// This is just a `try_despawn` alias.
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandSubscribeExtension for Commands<'w, 's> {
	fn subscribe<Out, OutError, S>(
		&mut self,
		observable_entity: Entity,
		destination_entity: Entity,
	) -> Entity
	where
		Out: SignalBound,
		OutError: SignalBound,
		S: ScheduleLabel,
	{
		let (subscribe_event, subscription_entity) =
			Subscribe::<Out, OutError>::new::<S>(observable_entity, destination_entity, self);

		self.trigger_targets(subscribe_event, observable_entity);

		subscription_entity
	}

	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).try_despawn();
	}
}
