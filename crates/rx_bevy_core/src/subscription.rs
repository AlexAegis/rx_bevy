use crate::SignalContext;

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike: SignalContext {
	fn unsubscribe(&mut self, context: &mut Self::Context);

	fn is_closed(&self) -> bool;
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'static + SubscriptionLike<Context = Self::Context>;
}
