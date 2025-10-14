use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionLike,
	context::{WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::{BevySubscriptionContextProvider, context::EntitySubscriptionContextAccessProvider};

use super::handle::UnscheduledEntitySubscriptionHandle;

pub struct UnscheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<ContextAccess> UnscheduledSubscriptionAllocator
	for UnscheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
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

impl<ContextAccess> WithSubscriptionContext
	for UnscheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<ContextAccess> Default for UnscheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
