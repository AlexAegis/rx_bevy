use std::marker::PhantomData;

use rx_core_traits::{
	Subscriber,
	context::{WithSubscriptionContext, allocator::DestinationAllocator},
	prelude::SubscriptionContext,
};

use crate::{
	BevySubscriptionContextProvider, EntitySubscriber,
	context::EntitySubscriptionContextAccessProvider,
};

pub struct SubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<ContextAccess> DestinationAllocator for SubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Shared<Destination>
		= EntitySubscriber<Destination, ContextAccess>
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

impl<ContextAccess> WithSubscriptionContext for SubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<ContextAccess> Default for SubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
