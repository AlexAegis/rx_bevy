use core::marker::PhantomData;
use std::sync::{Arc, RwLock};

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
		= Arc<RwLock<Destination>>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		Arc::new(RwLock::new(destination))
	}
}
