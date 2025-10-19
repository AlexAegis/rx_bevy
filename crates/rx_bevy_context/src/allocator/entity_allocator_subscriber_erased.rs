use bevy_ecs::hierarchy::ChildOf;
use rx_core_traits::{
	SignalBound, Subscriber, WithSubscriptionContext, allocator::ErasedDestinationAllocator,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, SharedErasedEntitySubscriber,
	SubscriberComponent,
};

#[derive(Default)]
pub struct ErasedSubscriberEntityAllocator;

impl ErasedDestinationAllocator for ErasedSubscriberEntityAllocator {
	type Shared<In, InError>
		= SharedErasedEntitySubscriber<In, InError>
	where
		In: SignalBound,
		InError: SignalBound;

	fn share<Destination>(
		destination: Destination,
		context: &mut BevySubscriptionContext<'_, '_>,
	) -> Self::Shared<Destination::In, Destination::InError>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		let subscription_entity = context.get_subscription_entity();
		let subscriber_entity = context.deferred_world.commands().spawn_empty().id();
		context
			.deferred_world
			.commands()
			.entity(subscriber_entity)
			.insert((
				ChildOf(subscription_entity),
				SubscriberComponent::new(destination, subscriber_entity),
			));

		SharedErasedEntitySubscriber::new(subscriber_entity)
	}
}

impl WithSubscriptionContext for ErasedSubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}
