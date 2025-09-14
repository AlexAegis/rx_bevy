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
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'static + SubscriptionLike<Context = Self::Context>,
	{
		let mut teardown: S = subscription.into();
		teardown.unsubscribe(context);
	}
}
