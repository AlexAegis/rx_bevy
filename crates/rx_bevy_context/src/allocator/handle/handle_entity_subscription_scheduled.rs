use bevy_ecs::{component::Component, entity::Entity};
use rx_core_traits::{
	SubscriptionContext, SubscriptionLike, SubscriptionNotification, Teardown, TeardownCollection,
	Tick, Tickable, WithSubscriptionContext, allocator::handle::ScheduledSubscriptionHandle,
};

use crate::{
	BevySubscriptionContextProvider,
	handle::{
		erased_subscription_add_notification_observer_on_insert,
		erased_subscription_unsubscribe_on_remove,
	},
};

use super::{UnscheduledEntitySubscriptionHandle, WeakEntitySubscriptionHandle};

pub trait ErasedEntitySubscriptionHandle {
	fn close_and_get_subscription_entity(&mut self) -> Entity;
}

#[derive(Component)]
#[component(on_insert=erased_subscription_add_notification_observer_on_insert, on_remove=erased_subscription_unsubscribe_on_remove::<Self>)]
pub struct ScheduledEntitySubscriptionHandle {
	subscription_entity: Entity,
	closed: bool,
}

impl ScheduledEntitySubscriptionHandle {
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed: false,
		}
	}
}

impl ErasedEntitySubscriptionHandle for ScheduledEntitySubscriptionHandle {
	fn close_and_get_subscription_entity(&mut self) -> Entity {
		self.closed = true;
		self.subscription_entity
	}
}

impl ScheduledSubscriptionHandle for ScheduledEntitySubscriptionHandle {
	type WeakHandle = WeakEntitySubscriptionHandle;
	type UnscheduledHandle = UnscheduledEntitySubscriptionHandle;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakEntitySubscriptionHandle::new(self.subscription_entity)
	}

	fn clone(&self) -> Self::UnscheduledHandle {
		UnscheduledEntitySubscriptionHandle::new(self.subscription_entity)
	}
}

impl WithSubscriptionContext for ScheduledEntitySubscriptionHandle {
	type Context = BevySubscriptionContextProvider;
}

impl Tickable for ScheduledEntitySubscriptionHandle {
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// Tick must not be stopped even if it's closed, in case a
		// downstream subscription is expecting it
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Tick(tick),
		);
	}
}

impl SubscriptionLike for ScheduledEntitySubscriptionHandle {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed = true;
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Unsubscribe,
			);
		}
	}
}

impl TeardownCollection for ScheduledEntitySubscriptionHandle {
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Add(teardown),
			);
		} else {
			teardown.execute(context);
		}
	}
}

impl Drop for ScheduledEntitySubscriptionHandle {
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context =
				BevySubscriptionContextProvider::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
