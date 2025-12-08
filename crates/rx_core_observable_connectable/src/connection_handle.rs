use std::sync::{Arc, RwLock};

use rx_core_traits::{SubscriptionLike, SubscriptionWithTeardown, Teardown, TeardownCollection};

/// Subscription that represents an active connection for a
/// [ConnectableObservable][crate::ConnectableObservable].
pub struct ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
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

impl<Subscription> SubscriptionLike for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.handle.is_closed()
	}
	#[inline]
	fn unsubscribe(&mut self) {
		self.handle.unsubscribe();
	}
}

impl<Subscription> TeardownCollection for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown) {
		self.handle.add_teardown(teardown);
	}
}

impl<Subscription> Drop for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			self.unsubscribe();
		}
	}
}
