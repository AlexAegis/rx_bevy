use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	ObservableSubscription, SubscriptionLike,
	context::{WithSubscriptionContext, allocator::ScheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	context::EntitySubscriptionContextAccessProvider,
	handle::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle},
};

pub struct ScheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<ContextAccess> ScheduledSubscriptionAllocator
	for ScheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type ScheduledHandle<Subscription>
		= ScheduledEntitySubscriptionHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync;

	type UnscheduledHandle<Subscription>
		= UnscheduledEntitySubscriptionHandle<Subscription>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_scheduled_subscription<Subscription>(
		subscription: Subscription,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::ScheduledHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		ScheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}

impl<ContextAccess> WithSubscriptionContext for ScheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<ContextAccess> Default for ScheduledEntitySubscriptionAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
