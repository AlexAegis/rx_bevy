use rx_core_traits::{
	Subscriber,
	context::{WithSubscriptionContext, allocator::DestinationAllocator},
	prelude::SubscriptionContext,
};

use crate::{BevySubscriptionContextProvider, EntitySubscriber};

#[derive(Default)]
pub struct SubscriberEntityAllocator;

impl DestinationAllocator for SubscriberEntityAllocator {
	type Shared<Destination>
		= EntitySubscriber<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		todo!("impl")
	}
}

impl WithSubscriptionContext for SubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}
