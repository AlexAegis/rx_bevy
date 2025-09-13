use crate::{SignalContext, SubscriptionCollection, SubscriptionLike};
use smallvec::SmallVec;

pub struct InnerSubscription<'c, Context: 'c> {
	is_closed: bool,
	finalizers:
		SmallVec<[Box<dyn SubscriptionLike<Context = <Self as SignalContext>::Context> + 'c>; 1]>,
}

impl<'c, Context: 'c> Default for InnerSubscription<'c, Context> {
	fn default() -> Self {
		Self {
			finalizers: SmallVec::new(),
			is_closed: false,
		}
	}
}

impl<'c, Context: 'c> SignalContext for InnerSubscription<'c, Context> {
	type Context = Context;
}

impl<'c, Context: 'c> SubscriptionLike for InnerSubscription<'c, Context> {
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

impl<'c, Context: 'c> SubscriptionCollection<'c> for InnerSubscription<'c, Context> {
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context> + 'c,
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
