use std::marker::PhantomData;

use super::SharedHeapSubscriberErased;
use crate::{
	SignalBound, Subscriber,
	context::{
		SubscriptionContext, WithSubscriptionContext, allocator::ErasedDestinationAllocator,
	},
};

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
		= SharedHeapSubscriberErased<In, InError, Context>
	where
		In: SignalBound,
		InError: SignalBound;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		SharedHeapSubscriberErased::new(destination)
	}
}
