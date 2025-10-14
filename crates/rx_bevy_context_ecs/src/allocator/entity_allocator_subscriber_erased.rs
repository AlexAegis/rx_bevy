use std::marker::PhantomData;

use rx_bevy_core::{
	SignalBound, Subscriber,
	context::{WithSubscriptionContext, allocator::ErasedDestinationAllocator},
};

use crate::{BevySubscriberContext, ErasedEntitySubscriber};

pub struct ErasedSubscriberEntityAllocator<'world, 'state> {
	_phantom_data: PhantomData<fn(&'world (), &'state ())>,
}

impl<'world, 'state> WithSubscriptionContext for ErasedSubscriberEntityAllocator<'world, 'state> {
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state> ErasedDestinationAllocator
	for ErasedSubscriberEntityAllocator<'world, 'state>
{
	type Shared<In, InError>
		= ErasedEntitySubscriber<'world, 'state, In, InError>
	where
		In: SignalBound,
		InError: SignalBound;

	fn share<Destination>(
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		todo!("impl")
	}
}
