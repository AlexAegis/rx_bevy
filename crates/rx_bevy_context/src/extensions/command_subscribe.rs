use std::any::TypeId;

use bevy_ecs::{entity::Entity, schedule::ScheduleLabel, system::Commands};
use rx_bevy_common::Clock;
use rx_core_traits::UpgradeableObserver;

use crate::{BevySubscriptionContextProvider, Subscribe};

pub trait CommandsUnsubscribeExtension {}

/// Provides functions to create subscriptions between two commands
pub trait CommandSubscribeExtension {
	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe<Destination, S, C>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
		S: ScheduleLabel,
		C: Clock;

	#[must_use = "It is advised to save the subscriptions entity reference somewhere to be able to unsubscribe from it at will."]
	fn subscribe_with_erased_schedule<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>;

	fn subscribe_unscheduled<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>;

	/// This is just a `try_despawn` alias.
	fn unsubscribe(&mut self, subscription_entity: Entity);
}

impl<'w, 's> CommandSubscribeExtension for Commands<'w, 's> {
	fn subscribe<Destination, Schedule, C>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
		Schedule: ScheduleLabel,
		C: Clock,
	{
		let (subscribe_event, subscription_entity) = Subscribe::<
			Destination::In,
			Destination::InError,
		>::new::<Destination, Schedule, C>(
			observable_entity, destination, self
		);

		// TODO(bevy-0.17): self.trigger(subscribe_event);
		let target = subscribe_event.observable_entity;
		self.trigger_targets(subscribe_event, target);

		subscription_entity
	}

	fn subscribe_with_erased_schedule<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
		schedule_component_type_id: TypeId,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
	{
		let (subscribe_event, subscription_entity) =
			Subscribe::<Destination::In, Destination::InError>::new_with_erased_schedule(
				observable_entity,
				destination,
				schedule_component_type_id,
				self,
			);

		// TODO(bevy-0.17): self.trigger(subscribe_event);
		let target = subscribe_event.observable_entity;
		self.trigger_targets(subscribe_event, target);

		subscription_entity
	}

	fn subscribe_unscheduled<Destination>(
		&mut self,
		observable_entity: Entity,
		destination: Destination,
	) -> Entity
	where
		Destination: 'static + UpgradeableObserver<Context = BevySubscriptionContextProvider>,
	{
		let (subscribe_event, subscription_entity) = Subscribe::<
			Destination::In,
			Destination::InError,
		>::new_unscheduled(
			observable_entity, destination, self
		);

		// TODO(bevy-0.17): self.trigger(subscribe_event);
		let target = subscribe_event.observable_entity;
		self.trigger_targets(subscribe_event, target);

		subscription_entity
	}

	fn unsubscribe(&mut self, subscription_entity: Entity) {
		self.entity(subscription_entity).try_despawn();
	}
}
