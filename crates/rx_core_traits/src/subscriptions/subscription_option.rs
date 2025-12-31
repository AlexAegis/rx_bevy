use crate::{SubscriptionLike, SubscriptionWithTeardown, TeardownCollection};

pub struct OptionSubscription<S>
where
	S: SubscriptionWithTeardown,
{
	subscription: Option<S>,
}

impl<S> OptionSubscription<S>
where
	S: SubscriptionWithTeardown,
{
	#[inline]
	pub fn new(subscription: Option<S>) -> Self {
		Self { subscription }
	}
}

impl<S> TeardownCollection for OptionSubscription<S>
where
	S: SubscriptionWithTeardown,
{
	fn add_teardown(&mut self, teardown: super::Teardown) {
		match self.subscription.as_mut() {
			Some(destination) => destination.add_teardown(teardown),
			None => teardown.execute(),
		}
	}
}

impl<S> SubscriptionLike for OptionSubscription<S>
where
	S: SubscriptionWithTeardown,
{
	fn is_closed(&self) -> bool {
		match self.subscription.as_ref() {
			Some(destination) => destination.is_closed(),
			None => true,
		}
	}

	fn unsubscribe(&mut self) {
		if let Some(mut destination) = self.subscription.take() {
			destination.unsubscribe();
		}
	}
}

impl<S> Drop for OptionSubscription<S>
where
	S: SubscriptionWithTeardown,
{
	#[inline]
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
