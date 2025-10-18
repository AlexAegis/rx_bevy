use bevy_ecs::entity::Entity;
use rx_core_traits::{
	SubscriptionLike,
	context::{WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::BevySubscriptionContextProvider;

use super::handle::UnscheduledEntitySubscriptionHandle;

#[derive(Default)]
pub struct UnscheduledEntitySubscriptionAllocator;

impl UnscheduledSubscriptionAllocator for UnscheduledEntitySubscriptionAllocator {
	type UnscheduledHandle<S>
		= UnscheduledEntitySubscriptionHandle<S>
	// TODO: Use a component
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync;

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
