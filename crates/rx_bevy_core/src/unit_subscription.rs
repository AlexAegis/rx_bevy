use crate::{SignalContext, SubscriptionCollection, SubscriptionLike};

impl SignalContext for () {
	type Context = ();
}

impl SubscriptionLike for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {}
}

impl SubscriptionCollection for () {
	fn add<S: SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Self::Context,
	) {
		let mut teardown: S = subscription.into();
		teardown.unsubscribe(context);
	}
}
