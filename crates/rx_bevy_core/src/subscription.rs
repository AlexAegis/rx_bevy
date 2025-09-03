use std::sync::{Arc, RwLock};

use smallvec::SmallVec;

#[cfg(feature = "channel_context")]
use crate::ChannelContext;

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike {
	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext);

	fn is_closed(&self) -> bool;

	fn add(
		&mut self,
		subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	);
}

pub enum Teardown {
	Fn(Box<dyn FnOnce()>),
	Sub(Box<dyn SubscriptionLike>),
}

impl Teardown {
	pub fn new<F: 'static + FnOnce()>(f: F) -> Self {
		Self::Fn(Box::new(f))
	}

	pub fn new_from_subscription(f: impl SubscriptionLike + 'static) -> Self {
		Self::Sub(Box::new(f))
	}

	pub(crate) fn call(self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		match self {
			Self::Fn(fun) => fun(),
			Self::Sub(mut sub) => {
				#[cfg(feature = "channel_context")]
				sub.unsubscribe(context);
				#[cfg(not(feature = "channel_context"))]
				sub.unsubscribe();
			}
		}
	}
}

impl From<Box<dyn SubscriptionLike>> for Teardown {
	fn from(subscription: Box<dyn SubscriptionLike>) -> Self {
		Self::Sub(subscription)
	}
}

impl<F> From<F> for Teardown
where
	F: 'static + FnOnce(),
{
	fn from(teardown: F) -> Self {
		Self::Fn(Box::new(teardown))
	}
}

pub struct InnerSubscription {
	is_closed: bool,
	finalizers: SmallVec<[Teardown; 1]>,
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

	pub fn add(
		&mut self,
		finalizer: impl Into<Teardown>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			#[cfg(feature = "channel_context")]
			finalizer.into().call(context);
			#[cfg(not(feature = "channel_context"))]
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

	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		let mut lock = self.inner.write().expect("to not be locked");

		#[cfg(feature = "channel_context")]
		lock.unsubscribe(context);

		#[cfg(not(feature = "channel_context"))]
		lock.unsubscribe();
	}

	fn add(
		&mut self,
		subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		let mut lock = self.inner.write().expect("to not be locked");

		#[cfg(feature = "channel_context")]
		lock.add(subscription, context);
		#[cfg(not(feature = "channel_context"))]
		lock.add(subscription);
	}
}

impl InnerSubscription {
	pub fn new(finalizer: impl Into<Teardown>) -> Self {
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

	pub fn add(
		&mut self,
		finalizer: impl Into<Teardown>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call(context);
		} else {
			self.finalizers.push(finalizer.into());
		}
	}
}

impl SubscriptionLike for InnerSubscription {
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] context: &mut ChannelContext) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				#[cfg(feature = "channel_context")]
				teardown.call(context);
				#[cfg(not(feature = "channel_context"))]
				teardown.call();
			}
		}
	}

	fn add(
		&mut self,
		subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] context: &mut ChannelContext,
	) {
		self.add(subscription, context);
	}
}

impl Drop for InnerSubscription {
	fn drop(&mut self) {
		#[cfg(not(feature = "channel_context"))]
		self.unsubscribe();

		#[cfg(feature = "channel_context")]
		if !self.is_closed() {
			panic!(
				"Dropped {} without unsubscribing first while feature 'channel_context' is enabled!",
				short_type_name::short_type_name::<Self>()
			)
		}
	}
}

impl SubscriptionLike for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, #[cfg(feature = "channel_context")] _context: &mut ChannelContext) {}

	fn add(
		&mut self,
		_subscription: Box<dyn SubscriptionLike>,
		#[cfg(feature = "channel_context")] _context: &mut ChannelContext,
	) {
	}
}
