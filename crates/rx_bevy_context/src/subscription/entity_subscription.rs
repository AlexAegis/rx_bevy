use bevy_ecs::entity::{ContainsEntity, Entity};
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	SchedulerHandle, SubscriptionData, SubscriptionLike, Teardown, TeardownCollection,
};

use crate::{RxBevyScheduler, RxBevySchedulerDespawnEntityExtension};

#[derive(RxSubscription)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct EntitySubscription {
	entity: Entity,
	despawn_scheduler: SchedulerHandle<RxBevyScheduler>,
	teardown: SubscriptionData,
}

impl EntitySubscription {
	pub fn new(entity: Entity, scheduler: SchedulerHandle<RxBevyScheduler>) -> Self {
		Self {
			entity,
			despawn_scheduler: scheduler,
			teardown: SubscriptionData::default(),
		}
	}
}

impl ContainsEntity for EntitySubscription {
	fn entity(&self) -> Entity {
		self.entity
	}
}

impl SubscriptionLike for EntitySubscription {
	#[inline]
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.teardown.unsubscribe();

			let mut scheduler = self.despawn_scheduler.lock();
			scheduler.schedule_despawn_entity(self.entity, None);
		}
	}
}

impl TeardownCollection for EntitySubscription {
	fn add_teardown(&mut self, teardown: Teardown) {
		if !self.is_closed() {
			self.teardown.add_teardown(teardown);
		} else {
			teardown.execute();
		}
	}
}

impl Drop for EntitySubscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
