use bevy_ecs::entity::Entity;
use rx_core_traits::{
	SubscriptionContext, SubscriptionLike, WithSubscriptionContext,
	allocator::UnscheduledSubscriptionAllocator,
};

use crate::BevySubscriptionContextProvider;

use super::handle::UnscheduledEntitySubscriptionHandle;

#[derive(Default)]
pub struct UnscheduledEntitySubscriptionAllocator;

impl UnscheduledSubscriptionAllocator for UnscheduledEntitySubscriptionAllocator {
	type UnscheduledHandle<Subscription>
		= UnscheduledEntitySubscriptionHandle<Subscription>
	where
		Subscription: 'static + SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<S>(
		subscription: S,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::UnscheduledHandle<S>
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		UnscheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}

impl WithSubscriptionContext for UnscheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}
