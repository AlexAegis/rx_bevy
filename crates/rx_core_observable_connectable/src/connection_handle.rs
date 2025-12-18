use std::sync::{Arc, RwLock};

use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::SubscriptionWithTeardown;

/// Subscription that represents an active connection for a
/// [ConnectableObservable][crate::ConnectableObservable].
#[derive(RxSubscription)]
#[rx_delegate_subscription_like_to_destination]
#[rx_delegate_teardown_collection]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	#[destination]
	handle: Arc<RwLock<Subscription>>,
}

impl<Subscription> ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	pub fn new(subscription: Subscription) -> Self {
		Self {
			handle: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> Clone for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			handle: self.handle.clone(),
		}
	}
}

impl<Subscription> Drop for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn drop(&mut self) {
		// Must not unsubscribe on drop, it's shared
	}
}
