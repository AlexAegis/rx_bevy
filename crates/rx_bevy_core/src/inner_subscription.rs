use crate::{SignalContext, SubscriptionCollection, SubscriptionLike};
use smallvec::SmallVec;

pub struct InnerSubscription<Context> {
	is_closed: bool,
	finalizers:
		SmallVec<[Box<dyn SubscriptionLike<Context = <Self as SignalContext>::Context>>; 1]>,
}

impl<Context> Default for InnerSubscription<Context> {
	fn default() -> Self {
		Self {
			finalizers: SmallVec::new(),
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

			for mut teardown in self.finalizers.drain(..) {
				teardown.unsubscribe(context);
			}
		}
	}
}

impl<Context> SubscriptionCollection for InnerSubscription<Context> {
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'static + SubscriptionLike<Context = Self::Context>,
	{
		let mut s: S = subscription.into();
		if self.is_closed() {
			// If this subscription is already closed, the added one is unsubscribed immediately
			s.unsubscribe(context);
		} else {
			self.finalizers.push(Box::new(s));
		}
	}
}
