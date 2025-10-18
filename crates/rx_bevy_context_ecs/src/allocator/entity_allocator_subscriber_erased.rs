use std::marker::PhantomData;

use rx_bevy_core::{
	SignalBound, Subscriber,
	context::{WithSubscriptionContext, allocator::ErasedDestinationAllocator},
	prelude::SubscriptionContext,
};

use crate::{
	BevySubscriptionContextProvider, ErasedEntitySubscriber,
	context::EntitySubscriptionContextAccessProvider,
};

pub struct ErasedSubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	_phantom_data: PhantomData<fn(ContextAccess)>,
}

impl<ContextAccess> ErasedDestinationAllocator for ErasedSubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Shared<In, InError>
		= ErasedEntitySubscriber<In, InError, ContextAccess>
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

impl<ContextAccess> WithSubscriptionContext for ErasedSubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	type Context = BevySubscriptionContextProvider<ContextAccess>;
}

impl<ContextAccess> Default for ErasedSubscriberEntityAllocator<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
