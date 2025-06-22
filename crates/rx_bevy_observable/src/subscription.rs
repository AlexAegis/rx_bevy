use std::ops::DerefMut;

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike {
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
		let teardown = finalizer.into();

		let is_already_closed = match &teardown {
			Teardown::Subscription(subscription) => subscription.is_closed(),
			_ => false,
		};

		if is_already_closed {
			teardown.call();
			Self {
				is_closed: true,
				finalizers: Vec::new(),
			}
		} else {
			Self {
				is_closed: is_already_closed,
				finalizers: vec![teardown],
			}
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

impl<T, Target> SubscriptionLike for T
where
	Target: SubscriptionLike,
	T: DerefMut<Target = Target>,
{
	fn is_closed(&self) -> bool {
		self.deref().is_closed()
	}

	fn unsubscribe(&mut self) {
		self.deref_mut().unsubscribe();
	}
}
