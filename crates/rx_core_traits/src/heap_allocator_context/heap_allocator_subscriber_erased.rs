use core::marker::PhantomData;
use std::sync::{Arc, RwLock};

use crate::{
	Signal, Subscriber,
	context::{
		SubscriptionContext, WithSubscriptionContext, allocator::ErasedDestinationAllocator,
	},
};

#[derive(Debug)]
pub struct ErasedSubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	_phantom_data: PhantomData<Context>,
}

impl<Context> WithSubscriptionContext for ErasedSubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> ErasedDestinationAllocator for ErasedSubscriberHeapAllocator<Context>
where
	Context: SubscriptionContext,
{
	type Shared<In, InError>
		= Arc<RwLock<dyn Subscriber<In = In, InError = InError, Context = Context> + Send + Sync>>
	where
		In: Signal,
		InError: Signal;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		Arc::new(RwLock::new(destination))
	}
}
