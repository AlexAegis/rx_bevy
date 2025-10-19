use rx_core_traits::{
	SignalBound, Subscriber, SubscriptionContext, WithSubscriptionContext,
	allocator::ErasedDestinationAllocator,
};

use crate::{BevySubscriptionContextProvider, ErasedEntitySubscriber};

#[derive(Default)]
pub struct ErasedSubscriberEntityAllocator;

impl ErasedDestinationAllocator for ErasedSubscriberEntityAllocator {
	type Shared<In, InError>
		= ErasedEntitySubscriber<In, InError>
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
		todo!("impl")
	}
}

impl WithSubscriptionContext for ErasedSubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}
