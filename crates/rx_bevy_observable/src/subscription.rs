/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
/// TODO: owned self in unsubscribe? Define it's behavior during drop instead
pub trait SubscriptionLike {
	/// TODO: Verify below comment, write test
	/// unsubscribe propagates backwards from the subscription back to the original observable's subscriber, stopping it.
	/// It should always be called before
	fn unsubscribe(&mut self);

	fn is_closed(&self) -> bool;
}

pub enum Teardown {
	Fn(Box<dyn FnOnce()>),
	Subscription(Box<dyn SubscriptionLike>),
}

impl Teardown {
	pub fn new<F: 'static + FnOnce()>(f: F) -> Self {
		Self::Fn(Box::new(f))
	}

	pub(crate) fn call(self) {
		match self {
			Self::Fn(fun) => fun(),
			Self::Subscription(mut subscription) => {
				subscription.unsubscribe();
			}
		}
	}
}

impl<S> From<S> for Teardown
where
	S: 'static + SubscriptionLike,
{
	fn from(value: S) -> Self {
		Self::Subscription(Box::new(value))
	}
}

pub struct Subscription {
	is_closed: bool,
	finalizers: Vec<Teardown>,
}

impl Subscription {
	pub fn new(finalizer: impl Into<Teardown>) -> Self {
		Self {
			is_closed: false,
			finalizers: vec![finalizer.into()],
		}
	}

	pub fn new_empty() -> Self {
		Self {
			is_closed: false,
			finalizers: Vec::new(),
		}
	}

	pub fn add(&mut self, finalizer: impl Into<Teardown>) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call();
		} else {
			self.finalizers.push(finalizer.into());
		}
	}
}

impl SubscriptionLike for Subscription {
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				teardown.call();
			}
		}
	}
}

impl Drop for Subscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
