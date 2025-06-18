use crate::Subscriber;

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

pub enum Teardown<Destination>
where
	Destination: 'static + Subscriber,
{
	Fn(Box<dyn FnOnce()>),
	Subscription(Box<Subscription<Destination>>),
}

impl<Destination> Teardown<Destination>
where
	Destination: 'static + Subscriber,
{
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

impl<Destination, F: 'static + FnOnce()> From<F> for Teardown<Destination>
where
	Destination: 'static + Subscriber,
{
	fn from(f: F) -> Self {
		Self::new(f)
	}
}

impl<Destination> From<Subscription<Destination>> for Teardown<Destination>
where
	Destination: 'static + Subscriber,
{
	fn from(value: Subscription<Destination>) -> Self {
		Self::Subscription(Box::new(value))
	}
}

pub struct Subscription<Destination>
where
	Destination: 'static + Subscriber,
{
	is_closed: bool,
	destination: Option<Destination>,
	finalizers: Vec<Teardown<Destination>>,
}

impl<Destination> Subscription<Destination>
where
	Destination: 'static + Subscriber,
{
	pub fn new(destination: Destination) -> Self {
		Self {
			is_closed: false,
			destination: Some(destination),
			finalizers: Vec::new(),
		}
	}

	pub fn add<Finalizer: 'static + FnOnce()>(
		&mut self,
		finalizer: impl Into<Teardown<Destination>>,
	) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call();
		} else {
			self.finalizers.push(finalizer.into());
		}
	}
}

impl<Destination> SubscriptionLike for Subscription<Destination>
where
	Destination: 'static + Subscriber,
{
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed {
			self.is_closed = true;

			if let Some(mut destination) = self.destination.take() {
				destination.unsubscribe();
			}

			for teardown in self.finalizers.drain(..) {
				teardown.call();
			}
		}
	}
}

impl<Destination> Drop for Subscription<Destination>
where
	Destination: 'static + Subscriber,
{
	fn drop(&mut self) {
		self.unsubscribe();
	}
}
