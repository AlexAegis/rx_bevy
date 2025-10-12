use std::{
	marker::PhantomData,
	sync::{Arc, RwLock},
};

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	ObservableSubscription, ScheduledSubscriptionAllocator, SubscriptionContext, SubscriptionLike,
	Teardown, UnscheduledSubscriptionHandle, WithSubscriptionContext,
};

use crate::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle};

pub struct ScheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> Default for ScheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> WithSubscriptionContext for ScheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> ScheduledSubscriptionAllocator for ScheduledEntitySubscriptionAllocator<Context>
where
	Context: SubscriptionContext,
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
		_context: &mut Self::Context,
	) -> Self::ScheduledHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync,
	{
		// TODO: Spawn subscription! Or spawn it somewhere else and just use the entity
		ScheduledEntitySubscriptionHandle::new(Entity::PLACEHOLDER)
	}
}
