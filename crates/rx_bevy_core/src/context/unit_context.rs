use crate::{
	ArcSubscriber, DropSafeSubscriptionContext, ErasedArcSubscriber, SignalBound, Subscriber,
	SubscriptionContext, WithSubscriptionContext,
};

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

	#[inline]
	fn create_context_to_unsubscribe_on_drop() -> Self {}
}

impl WithSubscriptionContext for () {
	type Context = ();
}
