use std::marker::PhantomData;

use rx_bevy_core::{
	SignalBound, Subscriber,
	context::{WithSubscriptionContext, allocator::ErasedDestinationAllocator},
};

use crate::{ContextWithCommands, ErasedEntitySubscriber};

pub struct ErasedSubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	_phantom_data: PhantomData<&'c fn(Context)>,
}

impl<'c, Context> WithSubscriptionContext for ErasedSubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	type Context = Context;
}

impl<'c, Context> ErasedDestinationAllocator for ErasedSubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	type Shared<In, InError>
		= ErasedEntitySubscriber<'c, In, InError, Context>
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
