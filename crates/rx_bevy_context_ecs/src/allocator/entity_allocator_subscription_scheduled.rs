use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	ObservableSubscription, SubscriptionLike,
	context::{WithSubscriptionContext, allocator::ScheduledSubscriptionAllocator},
	prelude::SubscriptionContext,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	handle::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle},
};

pub struct ScheduledEntitySubscriptionAllocator;

impl WithSubscriptionContext for ScheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}

impl ScheduledSubscriptionAllocator for ScheduledEntitySubscriptionAllocator {
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
