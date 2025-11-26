use super::{
	ErasedSubscriberHeapAllocator, ScheduledSubscriptionHeapAllocator, SubscriberHeapAllocator,
	UnscheduledSubscriptionHeapAllocator,
};
use crate::context::{
	DropSafeSubscriptionContext, SubscriptionContext, SubscriptionContextAccess,
	WithSubscriptionContext,
};

/// # Heap Context
///
/// Also known as the "unit context" or "no context". Using this, subscriptions
/// and shared subscribers will simply live on the heap behind an
/// [Arc][std::sync::Arc] and an [RwLock][std::sync::RwLock].
///
/// Use this if subscriptions don't need to be allocated through a context
/// reference. For example if you would want everything to be stored in an ECS.
impl SubscriptionContext for () {
	type Item<'w, 's> = ();
	type DropSafety = DropSafeSubscriptionContext;

	type DestinationAllocator = SubscriberHeapAllocator<Self>;
	type ErasedDestinationAllocator = ErasedSubscriberHeapAllocator<Self>;
	type ScheduledSubscriptionAllocator = ScheduledSubscriptionHeapAllocator<Self>;
	type UnscheduledSubscriptionAllocator = UnscheduledSubscriptionHeapAllocator<Self>;

	#[inline]
	fn create_context_to_unsubscribe_on_drop<'w, 's>() -> Self::Item<'w, 's> {}
}

impl WithSubscriptionContext for () {
	type Context = ();
}

impl SubscriptionContextAccess for () {
	type Context = ();
}
