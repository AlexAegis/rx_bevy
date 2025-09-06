use std::sync::{Arc, RwLock};

use smallvec::SmallVec;

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike<Context> {
	fn unsubscribe(&mut self, context: &mut Context);

	fn is_closed(&self) -> bool;
}

pub trait ExpandableSubscriptionLike<Context>: SubscriptionLike<Context> {
	fn add(&mut self, subscription: impl Into<Teardown<Context>>, context: &mut Context);
}

pub enum Teardown<Context> {
	Fn(Box<dyn FnOnce()>),
	Sub(Box<dyn SubscriptionLike<Context>>),
}

impl<Context> Teardown<Context> {
	pub fn new<F: 'static + FnOnce()>(f: F) -> Self {
		Self::Fn(Box::new(f))
	}

	pub fn new_from_subscription(f: impl SubscriptionLike<Context> + 'static) -> Self {
		Self::Sub(Box::new(f))
	}

	pub(crate) fn call(self, context: &mut Context) {
		match self {
			Self::Fn(fun) => fun(),
			Self::Sub(mut sub) => {
				sub.unsubscribe(context);
			}
		}
	}
}

impl<F, Context> From<F> for Teardown<Context>
where
	F: 'static + FnOnce(),
{
	fn from(teardown: F) -> Self {
		Self::Fn(Box::new(teardown))
	}
}

pub struct InnerSubscription {
	is_closed: bool,
	finalizers: SmallVec<[Teardown<()>; 1]>,
}

impl Default for InnerSubscription {
	fn default() -> Self {
		Self {
			finalizers: SmallVec::new(),
			is_closed: false,
		}
	}
}

#[derive(Clone)]
pub struct Subscription {
	inner: Arc<RwLock<InnerSubscription>>,
}

impl Subscription {
	pub fn new(finalizer: impl Into<Teardown<()>>) -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerSubscription::new(finalizer))),
		}
	}

	pub fn new_empty() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerSubscription::new_empty())),
		}
	}

	pub fn add(&mut self, finalizer: impl Into<Teardown<()>>, context: &mut ()) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call(context);
		} else {
			self.inner
				.write()
				.expect("not locked")
				.finalizers
				.push(finalizer.into());
		}
	}
}

impl SubscriptionLike<()> for Subscription {
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed
	}

	fn unsubscribe(&mut self, context: &mut ()) {
		let mut lock = self.inner.write().expect("to not be locked");

		lock.unsubscribe(context);
	}
}

impl ExpandableSubscriptionLike<()> for Subscription {
	fn add(&mut self, subscription: impl Into<Teardown<()>>, context: &mut ()) {
		let mut lock = self.inner.write().expect("to not be locked");

		lock.add_finalizer(subscription, context);
	}
}

impl InnerSubscription {
	pub fn new(finalizer: impl Into<Teardown<()>>) -> Self {
		let teardown = finalizer.into();
		let is_closed = matches!(&teardown, Teardown::Sub(sub) if sub.is_closed());

		Self {
			is_closed,
			finalizers: if is_closed {
				SmallVec::new()
			} else {
				smallvec::smallvec![teardown]
			},
		}
	}

	pub fn new_empty() -> Self {
		Self {
			is_closed: false,
			finalizers: SmallVec::new(),
		}
	}

	pub fn add_finalizer(&mut self, finalizer: impl Into<Teardown<()>>, context: &mut ()) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call(context);
		} else {
			self.finalizers.push(finalizer.into());
		}
	}
}

impl SubscriptionLike<()> for InnerSubscription {
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut ()) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				teardown.call(context);
			}
		}
	}
}

impl ExpandableSubscriptionLike<()> for InnerSubscription {
	fn add(&mut self, subscription: impl Into<Teardown<()>>, context: &mut ()) {
		self.add_finalizer(subscription, context);
	}
}

impl Drop for InnerSubscription {
	fn drop(&mut self) {
		self.unsubscribe(&mut ());
	}
}

impl<Context> SubscriptionLike<Context> for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Context) {}
}

impl<Context> ExpandableSubscriptionLike<Context> for () {
	fn add(&mut self, subscription: impl Into<Teardown<Context>>, context: &mut Context) {
		let teardown: Teardown<Context> = subscription.into();
		teardown.call(context);
	}
}
