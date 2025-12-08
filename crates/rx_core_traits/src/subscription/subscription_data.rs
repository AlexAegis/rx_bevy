use disqualified::ShortName;

use crate::{SubscriptionClosedFlag, SubscriptionLike, Teardown, TeardownCollection};
use std::{fmt::Debug, vec};

/// The base subscription implementation commonly used by other subscription
/// implementations.
///
/// This struct is just a collection of teardown closures, stored as the
/// closure itself.
///
/// This collection of closures represent the resources held by the
/// subscription. To release the resources the subscription must be unsubscribed
/// upon which the collection is drained, and the closures are called,
/// effectively dropping everything held by the subscription before the
/// subscription itself is dropped.
pub struct SubscriptionData {
	closed_flag: SubscriptionClosedFlag,
	finalizers: Vec<Box<dyn FnOnce() + Send + Sync>>,
}

impl SubscriptionData {
	pub fn new_with_teardown(teardown: Teardown) -> Self {
		if let Some(teardown) = teardown.take() {
			Self {
				closed_flag: false.into(),
				finalizers: vec![teardown],
			}
		} else {
			Self::default()
		}
	}
}

impl Default for SubscriptionData {
	fn default() -> Self {
		Self {
			finalizers: Vec::new(),
			closed_flag: false.into(),
		}
	}
}

impl SubscriptionLike for SubscriptionData {
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self) {
		if !self.is_closed() {
			self.closed_flag.close();

			for teardown in self.finalizers.drain(..) {
				(teardown)();
			}
		}
	}
}

impl TeardownCollection for SubscriptionData {
	fn add_teardown(&mut self, teardown: Teardown) {
		if self.is_closed() {
			// If this subscription is already closed, the newly added teardown
			// is immediately executed.
			teardown.execute();
		} else if let Some(teardown_fn) = teardown.take() {
			self.finalizers.push(teardown_fn);
		}
	}
}

impl Debug for SubscriptionData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {}, finalizers: {} }}",
			ShortName::of::<Self>(),
			self.is_closed(),
			self.finalizers.len()
		))
	}
}

impl Drop for SubscriptionData {
	fn drop(&mut self) {
		if !self.is_closed() {
			self.unsubscribe();
		}
	}
}
