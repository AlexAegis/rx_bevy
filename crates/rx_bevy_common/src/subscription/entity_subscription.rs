use bevy_ecs::entity::{ContainsEntity, Entity};
use rx_core_common::{
	Scheduler, SchedulerHandle, SubscriptionWithTeardown, Teardown, TeardownCollection,
	TeardownCollectionExtension,
};
use rx_core_macro_subscription_derive::RxSubscription;

use crate::{RxBevyScheduler, RxBevySchedulerDespawnEntityExtension};

#[derive(RxSubscription)]
#[rx_delegate_subscription_like_to_destination]
#[rx_skip_unsubscribe_on_drop_impl] // This is technically shared
pub struct EntitySubscription {
	entity: Entity,
	#[destination]
	subscriber: Box<dyn SubscriptionWithTeardown + Send + Sync>,
}

impl EntitySubscription {
	pub fn new<S>(
		entity: Entity,
		mut subscription: S,
		scheduler: SchedulerHandle<RxBevyScheduler>,
	) -> Self
	where
		S: 'static + SubscriptionWithTeardown + Send + Sync,
	{
		let despawn_invoke_id = {
			let mut scheduler = scheduler.lock();
			let invoke_id = scheduler.generate_invoke_id();

			scheduler.schedule_invoked_despawn_entity(entity, invoke_id);
			invoke_id
		};

		let scheduler_clone = scheduler.clone();
		subscription.add_fn(move || {
			scheduler_clone.lock().invoke(despawn_invoke_id);
		});
		Self {
			entity,
			subscriber: Box::new(subscription),
		}
	}
}

impl ContainsEntity for EntitySubscription {
	#[inline]
	fn entity(&self) -> Entity {
		self.entity
	}
}

impl TeardownCollection for EntitySubscription {
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.subscriber.add_teardown(teardown);
	}
}

impl Drop for EntitySubscription {
	fn drop(&mut self) {
		// Should do nothing, the actualy subscription is shared through the entity
	}
}
