use core::marker::PhantomData;

use super::handle::UnscheduledHeapSubscriptionHandle;
use crate::{
	SubscriptionWithTeardown,
	context::{
		SubscriptionContext, WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator,
	},
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
		= UnscheduledHeapSubscriptionHandle<Subscription>
	where
		Subscription: 'static + SubscriptionWithTeardown<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<Subscription>(
		subscription: Subscription,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::UnscheduledHandle<Subscription>
	where
		Subscription: 'static + SubscriptionWithTeardown<Context = Self::Context> + Send + Sync,
	{
		UnscheduledHeapSubscriptionHandle::new(subscription)
	}
}
