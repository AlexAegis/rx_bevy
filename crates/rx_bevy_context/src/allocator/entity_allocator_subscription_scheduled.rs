use rx_core_traits::{
	ObservableSubscription, SubscriptionLike, WithSubscriptionContext,
	allocator::ScheduledSubscriptionAllocator,
};

use crate::{
	BevySubscriptionContextProvider,
	handle::{ScheduledEntitySubscriptionHandle, UnscheduledEntitySubscriptionHandle},
};

#[derive(Default)]
pub struct ScheduledEntitySubscriptionAllocator;

impl ScheduledSubscriptionAllocator for ScheduledEntitySubscriptionAllocator {
	type ScheduledHandle<Subscription>
		= ScheduledEntitySubscriptionHandle
	where
		Subscription: 'static + ObservableSubscription<Context = Self::Context> + Send + Sync;

	type UnscheduledHandle<Subscription>
		= UnscheduledEntitySubscriptionHandle
	where
		Subscription: 'static + SubscriptionLike<Context = Self::Context> + Send + Sync;
}

impl WithSubscriptionContext for ScheduledEntitySubscriptionAllocator {
	type Context = BevySubscriptionContextProvider;
}
