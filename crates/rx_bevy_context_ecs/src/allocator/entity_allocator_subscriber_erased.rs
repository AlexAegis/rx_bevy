use std::marker::PhantomData;

use rx_bevy_core::{
	SignalBound, Subscriber,
	context::{WithSubscriptionContext, allocator::ErasedDestinationAllocator},
	prelude::SubscriptionContext,
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider, ErasedEntitySubscriber};

pub struct ErasedSubscriberEntityAllocator;

impl WithSubscriptionContext for ErasedSubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}

impl ErasedDestinationAllocator for ErasedSubscriberEntityAllocator {
	type Shared<In, InError>
		= ErasedEntitySubscriber<In, InError>
	where
		In: SignalBound,
		InError: SignalBound;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		todo!("impl")
	}
}
