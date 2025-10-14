use std::marker::PhantomData;

use super::handle::{ScheduledHeapSubscriptionHandle, UnscheduledHeapSubscriptionHandle};
use crate::{
	ObservableSubscription, SubscriptionLike,
	context::{
		SubscriptionContext, WithSubscriptionContext, allocator::ScheduledSubscriptionAllocator,
	},
};

pub struct ScheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> Default for ScheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> WithSubscriptionContext for ScheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> ScheduledSubscriptionAllocator for ScheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type ScheduledHandle<Subscription>
		= ScheduledHeapSubscriptionHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync;

	type UnscheduledHandle<Subscription>
		= UnscheduledHeapSubscriptionHandle<Subscription>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_scheduled_subscription<Subscription>(
		subscription: Subscription,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::ScheduledHandle<Subscription>
	where
		Subscription: ObservableSubscription<Context = Self::Context> + Send + Sync,
	{
		ScheduledHeapSubscriptionHandle::new(subscription)
	}
}
