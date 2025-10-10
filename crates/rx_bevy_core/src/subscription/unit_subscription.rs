use crate::{
	DropSafeSignalContext, ObservableOutput, ObserverInput, SignalContext, SubscriptionLike,
	Teardown, Tickable, WithContext,
};

impl ObserverInput for () {
	type In = ();
	type InError = ();
}

impl ObservableOutput for () {
	type Out = ();
	type OutError = ();
}

impl SignalContext for () {
	type DropSafety = DropSafeSignalContext;

	#[inline]
	fn create_context_to_unsubscribe_on_drop() -> Self {}
}

impl WithContext for () {
	type Context = ();
}

impl Tickable for () {
	fn tick(&mut self, _tick: crate::Tick, _context: &mut Self::Context) {}
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
		teardown.execute(context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Self::create_context_to_unsubscribe_on_drop()
	}
}
