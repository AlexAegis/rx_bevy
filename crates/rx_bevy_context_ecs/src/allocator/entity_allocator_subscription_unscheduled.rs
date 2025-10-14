use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike,
	context::{WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider};

use super::handle::UnscheduledEntitySubscriptionHandle;

pub struct UnscheduledEntitySubscriptionAllocator;

impl WithSubscriptionContext for UnscheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}

impl UnscheduledSubscriptionAllocator for UnscheduledEntitySubscriptionAllocator {
	type UnscheduledHandle<S>
		= UnscheduledEntitySubscriptionHandle<S>
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<S>(
		subscription: S,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::UnscheduledHandle<S>
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		UnscheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}
