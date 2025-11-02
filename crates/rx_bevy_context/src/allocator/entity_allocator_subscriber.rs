use bevy_ecs::hierarchy::ChildOf;
use rx_core_traits::{Subscriber, WithSubscriptionContext, allocator::DestinationAllocator};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, SharedEntitySubscriber,
	SubscriberComponent,
};

#[deprecated = "maybe giving these an entity is a bad idea, it is with the switch, has to be tried with subjects too"]
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
		let subscription_entity = context.get_subscription_entity();

		let mut commands = context.deferred_world.commands();
		let subscriber_entity = commands.spawn(ChildOf(subscription_entity)).id();
		commands
			.entity(subscriber_entity)
			.insert(SubscriberComponent::new(destination, subscriber_entity));

		SharedEntitySubscriber::new(subscriber_entity)
	}
}

impl WithSubscriptionContext for SubscriberEntityAllocator {
	type Context = BevySubscriptionContextProvider;
}
