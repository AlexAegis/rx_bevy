use std::marker::PhantomData;

use bevy_ecs::entity::Entity;
use rx_bevy_core::{
	ObservableSubscription, SubscriptionLike,
	context::{WithSubscriptionContext, allocator::ScheduledSubscriptionAllocator},
};

use crate::{
	BevySubscriberContext,
	handle::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle},
};

pub struct ScheduledEntitySubscriptionAllocator<'world, 'state> {
	_phantom_data: PhantomData<fn(&'world (), &'state ())>,
}

impl<'world, 'state> Default for ScheduledEntitySubscriptionAllocator<'world, 'state> {
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<'world, 'state> WithSubscriptionContext
	for ScheduledEntitySubscriptionAllocator<'world, 'state>
{
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state> ScheduledSubscriptionAllocator
	for ScheduledEntitySubscriptionAllocator<'world, 'state>
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
