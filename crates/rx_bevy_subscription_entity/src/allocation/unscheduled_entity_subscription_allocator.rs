use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	SubscriptionContext, SubscriptionLike, Teardown, UnscheduledSubscriptionAllocator,
	UnscheduledSubscriptionHandle, WithSubscriptionContext,
};

use crate::UnscheduledEntitySubscriptionHandle;

pub struct UnscheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> Default for UnscheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> WithSubscriptionContext for UnscheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> UnscheduledSubscriptionAllocator for UnscheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
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
