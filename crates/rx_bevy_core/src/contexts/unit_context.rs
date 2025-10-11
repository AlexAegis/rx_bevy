use crate::{
	ArcSubscriber, DropSafeSignalContext, ErasedArcSubscriber, SignalBound, SignalContext,
	Subscriber, WithContext,
};

impl SignalContext for () {
	type DropSafety = DropSafeSignalContext;

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

impl WithContext for () {
	type Context = ();
}
