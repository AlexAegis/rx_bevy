use crate::{
	DropContext, DropSafeSignalContext, SignalContext, SubscriptionCollection, SubscriptionLike,
	Teardown,
};

impl DropContext for () {
	type DropSafety = DropSafeSignalContext;

	#[inline]
	fn get_context_for_drop() -> Self {}
}

impl SignalContext for () {
	type Context = ();
}

impl SubscriptionLike for () {
	#[inline]
	fn is_closed(&self) -> bool {
		true
	}

	#[inline]
	fn unsubscribe(&mut self, _context: &mut Self::Context) {}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::get_context_for_drop()
	}
}

impl SubscriptionCollection for () {
	#[inline]
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		teardown.call(context);
	}
}
