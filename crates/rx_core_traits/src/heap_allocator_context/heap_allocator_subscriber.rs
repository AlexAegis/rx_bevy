use std::marker::PhantomData;

use super::SharedHeapSubscriber;
use crate::{
	Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::DestinationAllocator},
};

pub struct SubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<Context>,
}

impl<Context> WithSubscriptionContext for SubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> DestinationAllocator for SubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Shared<Destination>
		= SharedHeapSubscriber<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		SharedHeapSubscriber::new(destination)
	}
}
