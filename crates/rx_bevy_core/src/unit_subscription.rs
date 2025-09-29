use crate::{DropContext, DropSafeSignalContext, SignalContext, SubscriptionLike, Teardown};

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
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		teardown.call(context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Self::get_context_for_drop()
	}
}
