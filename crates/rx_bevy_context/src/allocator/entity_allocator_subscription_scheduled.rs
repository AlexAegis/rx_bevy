use bevy_ecs::entity::Entity;
use rx_core_traits::{
	ObservableSubscription, SubscriptionLike,
	context::{WithSubscriptionContext, allocator::ScheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::{
	BevySubscriptionContextProvider,
	handle::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle},
};

#[derive(Default)]
pub struct ScheduledEntitySubscriptionAllocator;

impl ScheduledSubscriptionAllocator for ScheduledEntitySubscriptionAllocator {
	type ScheduledHandle<Subscription>
		= ScheduledEntitySubscriptionHandle<Subscription>
	where
		Subscription: 'static + ObservableSubscription<Context = Self::Context> + Send + Sync;

	type UnscheduledHandle<Subscription>
		= UnscheduledEntitySubscriptionHandle<Subscription>
	where
		Subscription: 'static + SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_scheduled_subscription<Subscription, Schedule>(
		subscription: Subscription,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::ScheduledHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		ScheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}

impl WithSubscriptionContext for ScheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}
