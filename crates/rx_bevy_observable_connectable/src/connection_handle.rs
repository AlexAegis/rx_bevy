use std::sync::{Arc, RwLock};

use rx_bevy_core::{SubscriptionLike, Teardown, WithSubscriptionContext};

/// Subscription that represents an active connection for a
/// [ConnectableObservable][crate::ConnectableObservable].
pub struct ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	handle: Arc<RwLock<Subscription>>,
}

impl<Subscription> ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	pub fn new(subscription: Subscription) -> Self {
		Self {
			handle: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> Clone for ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	fn clone(&self) -> Self {
		Self {
			handle: self.handle.clone(),
		}
	}
}

impl<Subscription> WithSubscriptionContext for ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	type Context = Subscription::Context;
}

impl<Subscription> SubscriptionLike for ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.handle.read() {
			lock.is_closed()
		} else {
			panic!("ConnectionHandle's lock is poisoned!")
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Ok(mut lock) = self.handle.write() {
			lock.unsubscribe(context);
		} else {
			panic!("ConnectionHandle's lock is poisoned!");
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if let Ok(mut lock) = self.handle.write() {
			lock.add_teardown(teardown, context);
		} else {
			panic!("ConnectionHandle's lock is poisoned!");
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Ok(mut lock) = self.handle.write() {
			lock.get_context_to_unsubscribe_on_drop()
		} else {
			panic!("ConnectionHandle's lock is poisoned!");
		}
	}
}

impl<Subscription> Drop for ConnectionHandle<Subscription>
where
	Subscription: SubscriptionLike,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
