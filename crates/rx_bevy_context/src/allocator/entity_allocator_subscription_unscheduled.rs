use rx_core_traits::{
	SubscriptionLike, WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider, UnscheduledSubscriptionComponent,
};

use super::handle::UnscheduledEntitySubscriptionHandle;

#[derive(Default)]
pub struct UnscheduledEntitySubscriptionAllocator;

impl UnscheduledSubscriptionAllocator for UnscheduledEntitySubscriptionAllocator {
	type UnscheduledHandle<Subscription>
		= UnscheduledEntitySubscriptionHandle
	where
		Subscription: 'static + SubscriptionLike<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<S>(
		subscription: S,
		context: &mut BevySubscriptionContext<'_, '_>,
	) -> Self::UnscheduledHandle<S>
	where
		S: 'static + SubscriptionLike<Context = Self::Context> + Send + Sync,
	{
		let subscription_entity = context.deferred_world.commands().spawn_empty().id();

		context
			.deferred_world
			.commands()
			.entity(subscription_entity)
			.insert(UnscheduledSubscriptionComponent::new(
				subscription,
				subscription_entity,
			));

		UnscheduledEntitySubscriptionHandle::new(subscription_entity)
	}
}

impl WithSubscriptionContext for UnscheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}
