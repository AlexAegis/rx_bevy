use crate::{
	ArcSubscriber, DropSafeSubscriptionContext, ErasedArcSubscriber, ObservableSubscription,
	ScheduledSubscriptionHeapAllocator, SignalBound, Subscriber, SubscriptionContext,
	SubscriptionLike, UnscheduledSubscriptionHeapAllocator, WithSubscriptionContext,
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
	type DropSafety = DropSafeSubscriptionContext;

	type Sharer<Destination>
		= ArcSubscriber<Destination>
	where
		Destination: 'static + Subscriber<Context = Self> + Send + Sync;

	type ErasedSharer<In, InError>
		= ErasedArcSubscriber<In, InError, Self>
	where
		In: SignalBound,
		InError: SignalBound;

	type ScheduledSubscriptionAllocator<Subscription>
		= ScheduledSubscriptionHeapAllocator<Self>
	where
		Subscription: 'static + ObservableSubscription<Context = Self> + Send + Sync;

	type UnscheduledSubscriptionAllocator<Subscription>
		= UnscheduledSubscriptionHeapAllocator<Self>
	where
		Subscription: 'static + SubscriptionLike<Context = Self> + Send + Sync;

	#[inline]
	fn create_context_to_unsubscribe_on_drop() -> Self {}
}

impl WithSubscriptionContext for () {
	type Context = ();
}
