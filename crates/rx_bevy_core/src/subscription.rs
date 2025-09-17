use crate::{SignalContext, Teardown};

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike: SignalContext {
	fn unsubscribe(&mut self, context: &mut Self::Context);

	fn is_closed(&self) -> bool;

	// TODO: Rename, to emphazise it's for dropping only
	fn get_unsubscribe_context(&mut self) -> Option<Self::Context>;
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>;

	fn add_fn<F>(&mut self, f: F, context: &mut Self::Context)
	where
		F: 'static + FnOnce(&mut Self::Context),
		Self: Sized,
	{
		let teardown = Teardown::<Self, Self::Context>::new(f);
		self.add(teardown, context);
	}
}
