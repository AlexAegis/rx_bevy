use bevy_ecs::hierarchy::ChildOf;
use rx_core_traits::{
	SubscriptionWithTeardown, WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator,
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
		Subscription: 'static + SubscriptionWithTeardown<Context = Self::Context> + Send + Sync;

	fn allocate_unscheduled_subscription<S>(
		subscription: S,
		context: &mut BevySubscriptionContext<'_, '_>,
	) -> Self::UnscheduledHandle<S>
	where
		S: 'static + SubscriptionWithTeardown<Context = Self::Context> + Send + Sync,
	{
		let contextual_subscription_entity = context.get_subscription_entity();
		let unscheduled_subscription_entity = context.deferred_world.commands().spawn_empty().id();

		context
			.deferred_world
			.commands()
			.entity(unscheduled_subscription_entity)
			.insert((
				ChildOf(contextual_subscription_entity),
				UnscheduledSubscriptionComponent::new(
					subscription,
					unscheduled_subscription_entity,
				),
			));

		UnscheduledEntitySubscriptionHandle::new(unscheduled_subscription_entity)
	}
}

impl WithSubscriptionContext for UnscheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}
