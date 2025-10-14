use std::marker::PhantomData;

use rx_bevy_core::{
	Subscriber,
	context::{WithSubscriptionContext, allocator::DestinationAllocator},
};

use crate::{BevySubscriberContext, EntitySubscriber};

pub struct SubscriberEntityAllocator<'world, 'state> {
	_phantom_data: PhantomData<fn(&'world (), &'state ())>,
}

impl<'world, 'state> WithSubscriptionContext for SubscriberEntityAllocator<'world, 'state> {
	type Context = BevySubscriberContext<'world, 'state>;
}

impl<'world, 'state> DestinationAllocator for SubscriberEntityAllocator<'world, 'state> {
	type Shared<Destination>
		= EntitySubscriber<'world, 'state, Destination>
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
