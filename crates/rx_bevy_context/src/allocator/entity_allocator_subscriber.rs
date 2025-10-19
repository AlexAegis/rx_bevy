use rx_core_traits::{Subscriber, WithSubscriptionContext, allocator::DestinationAllocator};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, SharedEntitySubscriber,
	SubscriberComponent,
};

#[derive(Default)]
pub struct SubscriberEntityAllocator;

impl DestinationAllocator for SubscriberEntityAllocator {
	/// Why not just use [SubscriberComponent]? They do look similar, but there
	/// is a key difference: EntitySubscriber does not own its destination, it's
	/// just an entity, which makes it trivially clonable.
	type Shared<Destination>
		= SharedEntitySubscriber<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync;

	fn share<Destination>(
		destination: Destination,
		context: &mut BevySubscriptionContext<'_, '_>,
	) -> Self::Shared<Destination>
	where
		Destination: 'static + Subscriber<Context = Self::Context> + Send + Sync,
	{
		let subscriber_entity = context.deferred_world.commands().spawn_empty().id();
		context
			.deferred_world
			.commands()
			.entity(subscriber_entity)
			.insert(SubscriberComponent::new(destination, subscriber_entity));

		SharedEntitySubscriber::new(subscriber_entity)
	}
}

impl WithSubscriptionContext for SubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}
