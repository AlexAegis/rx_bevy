use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{SubscriptionLike, Teardown, TeardownCollection};

/// A [InertSubscription] is a permanently closed [Subscription] that immediately
/// runs any [Teardown] you may add into it.
/// It is used for [Observable]s that emit all their values, complete and
/// unsubscribe immediately on subscribe.
#[derive(RxSubscription)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct InertSubscription;

impl InertSubscription {
	pub fn new(mut destination: impl SubscriptionLike + 'static + Send + Sync) -> Self {
		// Immediately unsubscribes if it's not already closed.
		if !destination.is_closed() {
			destination.unsubscribe();
		}
		Self
	}
}

impl SubscriptionLike for InertSubscription {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self) {
		// Does not need to do anything on unsubscribe
	}
}

impl TeardownCollection for InertSubscription {
	fn add_teardown(&mut self, teardown: Teardown) {
		// The added teardown is executed immediately as this subscription is always closed.
		teardown.execute();
	}
}

impl Drop for InertSubscription {
	fn drop(&mut self) {
		// Does not need to do anything on drop, as it contains nothing.
	}
}
