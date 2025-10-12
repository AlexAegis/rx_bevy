use std::marker::PhantomData;

use crate::{
	SubscriptionContext, SubscriptionLike, UnscheduledArcSubscriptionHandle,
	UnscheduledSubscriptionAllocator, WithSubscriptionContext,
};

pub struct UnscheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<fn(Context)>,
}

impl<Context> Default for UnscheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> WithSubscriptionContext for UnscheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> UnscheduledSubscriptionAllocator for UnscheduledSubscriptionHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type UnscheduledHandle<Subscription>
		= UnscheduledArcSubscriptionHandle<Subscription>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<Subscription>(
		subscription: Subscription,
		_context: &mut Self::Context,
	) -> Self::UnscheduledHandle<Subscription>
	where
		Subscription: SubscriptionLike<Context = Self::Context> + Send + Sync,
	{
		UnscheduledArcSubscriptionHandle::new(subscription)
	}
}
