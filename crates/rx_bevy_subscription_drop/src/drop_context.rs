use rx_bevy_core::SignalContext;

/// For subscriptions where the [Subscription] itself contains everything
/// needed for it to unsubscribe, this trait can enable unsubscribe-on-drop
/// behavior.
pub trait DropContextFromSubscription: SignalContext {
	fn get_unsubscribe_context(&mut self) -> Self::Context;
}

impl DropContextFromSubscription for () {
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		()
	}
}

/// In addition to [ContextFromSubscription], this trait denotes contexts for
/// for dropped [Subscription]s. For example when the context is just `()`.
pub trait DropContext {
	fn get_context_for_drop() -> Self;
}

impl DropContext for () {
	fn get_context_for_drop() -> Self {
		()
	}
}
