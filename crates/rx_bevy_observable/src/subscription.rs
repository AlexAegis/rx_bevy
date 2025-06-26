use std::sync::{Arc, RwLock};

use smallvec::SmallVec;

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike {
	fn unsubscribe(&mut self);

	fn is_closed(&self) -> bool;

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike);
}

pub enum Teardown {
	Fn(Box<dyn FnOnce()>),
	Subscription(Box<dyn SubscriptionLike>),
	Sub(&'static mut dyn SubscriptionLike),
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
			Self::Sub(sub) => {
				sub.unsubscribe();
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

pub struct InnerSubscription {
	is_closed: bool,
	finalizers: SmallVec<[Teardown; 1]>,
}

#[derive(Clone)]
pub struct Subscription {
	inner: Arc<RwLock<InnerSubscription>>,
}

impl Subscription {
	pub fn new(finalizer: impl Into<Teardown>) -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerSubscription::new(finalizer))),
		}
	}

	pub fn new_empty() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerSubscription::new_empty())),
		}
	}

	pub fn add(&mut self, finalizer: impl Into<Teardown>) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call();
		} else {
			self.inner
				.write()
				.expect("not locked")
				.finalizers
				.push(finalizer.into());
		}
	}
}

impl SubscriptionLike for Subscription {
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed
	}

	fn unsubscribe(&mut self) {
		self.inner.write().expect("to not be locked").unsubscribe();
	}

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.inner
			.write()
			.expect("to not be locked")
			.add(Teardown::Sub(subscription));
	}
}

impl InnerSubscription {
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
				finalizers: SmallVec::new(),
			}
		} else {
			Self {
				is_closed: is_already_closed,
				finalizers: smallvec::smallvec![teardown],
			}
		}
	}

	pub fn new_empty() -> Self {
		Self {
			is_closed: false,
			finalizers: SmallVec::new(),
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

impl SubscriptionLike for InnerSubscription {
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

	fn add(&mut self, subscription: &'static mut dyn SubscriptionLike) {
		self.add(Teardown::Sub(subscription));
	}
}

impl Drop for InnerSubscription {
	fn drop(&mut self) {
		self.unsubscribe();
	}
}

impl SubscriptionLike for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self) {}

	fn add(&mut self, _subscription: &'static mut dyn SubscriptionLike) {}
}
