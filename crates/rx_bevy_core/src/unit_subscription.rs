use crate::{SignalContext, SubscriptionCollection, SubscriptionLike, Teardown};

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
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		teardown.call(context);
	}
}
