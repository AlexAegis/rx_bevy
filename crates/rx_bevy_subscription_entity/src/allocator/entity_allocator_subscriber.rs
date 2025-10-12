use std::marker::PhantomData;

use rx_bevy_core::{
	Subscriber,
	context::{SubscriptionContext, WithSubscriptionContext, allocator::DestinationAllocator},
};

use crate::{ContextWithCommands, EntitySubscriber};

pub struct SubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	_phantom_data: PhantomData<&'c Context>,
}

impl<'c, Context> WithSubscriptionContext for SubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	type Context = Context;
}

impl<'c, Context> DestinationAllocator for SubscriberEntityAllocator<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	type Shared<Destination>
		= EntitySubscriber<'c, Destination, Context>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		todo!("impl")
	}
}
