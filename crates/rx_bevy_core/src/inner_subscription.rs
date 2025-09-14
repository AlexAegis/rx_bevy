use crate::{SignalContext, SubscriptionCollection, SubscriptionLike, Teardown};

pub struct InnerSubscription<Context> {
	is_closed: bool,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> Default for InnerSubscription<Context> {
	fn default() -> Self {
		Self {
			finalizers: Vec::new(),
			is_closed: false,
		}
	}
}

impl<Context> SignalContext for InnerSubscription<Context> {
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerSubscription<Context> {
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				(teardown)(context);
			}
		}
	}
}

impl<Context> SubscriptionCollection for InnerSubscription<Context> {
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		if self.is_closed() {
			// If this subscription is already closed, the added one is unsubscribed immediately
			teardown.take()(context);
		} else {
			self.finalizers.push(teardown.take());
		}
	}
}
