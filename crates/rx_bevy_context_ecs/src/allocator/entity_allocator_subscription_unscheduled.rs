use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike,
	context::{WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator},
};

use crate::BevySubscriberContext;

use super::handle::UnscheduledEntitySubscriptionHandle;

pub struct UnscheduledEntitySubscriptionAllocator<'world, 'state> {
	_phantom_data: PhantomData<fn(&BevySubscriberContext<'world, 'state>)>,
}

impl<'world, 'state> Default for UnscheduledEntitySubscriptionAllocator<'world, 'state> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<'world, 'state> WithSubscriptionContext
	for UnscheduledEntitySubscriptionAllocator<'world, 'state>
{
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state> UnscheduledSubscriptionAllocator
	for UnscheduledEntitySubscriptionAllocator<'world, 'state>
{
	type UnscheduledHandle<S>
		= UnscheduledEntitySubscriptionHandle<S>
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<S>(
		subscription: S,
		_context: &mut Self::Context,
	) -> Self::UnscheduledHandle<S>
	where
		S: SubscriptionLike<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		UnscheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}
