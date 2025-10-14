use std::marker::PhantomData;

use rx_bevy_core::{
	Subscriber,
	context::{WithSubscriptionContext, allocator::DestinationAllocator},
	prelude::SubscriptionContext,
};

use crate::{BevySubscriptionContext, BevySubscriptionContextProvider, EntitySubscriber};

pub struct SubscriberEntityAllocator;

impl WithSubscriptionContext for SubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}

impl DestinationAllocator for SubscriberEntityAllocator {
	type Shared<Destination>
		= EntitySubscriber<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		todo!("impl")
	}
}
