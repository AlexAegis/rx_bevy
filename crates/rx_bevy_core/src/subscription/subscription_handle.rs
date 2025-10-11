use std::sync::{Arc, RwLock};

use crate::{SubscriptionLike, Tick, Tickable, TickableSubscription, WithContext};

/// Subscriptions are made cloneable through a smart pointer.
/// Subscriptions have to be clonable to allow their storage to keep them alive
/// and to still let them be unsubscribable and tickable from other places.
pub struct SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	handle: Arc<RwLock<Subscription>>,
}

impl<Subscription> SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	pub fn new(subscription: Subscription) -> Self {
		Self {
			handle: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> Clone for SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	fn clone(&self) -> Self {
		Self {
			handle: self.handle.clone(),
		}
	}
}

impl<Subscription> WithContext for SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	type Context = Subscription::Context;
}

impl<Subscription> Tickable for SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if let Ok(mut lock) = self.handle.write() {
			lock.tick(tick, context);
		} else {
			panic!("SubscriptionHandle's lock is poisoned!");
		}
	}
}

impl<Subscription> SubscriptionLike for SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.handle.read() {
			lock.is_closed()
		} else {
			panic!("SubscriptionHandle's lock is poisoned!")
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if let Ok(mut lock) = self.handle.write() {
			lock.unsubscribe(context);
		} else {
			panic!("SubscriptionHandle's lock is poisoned!");
		}
	}

	fn add_teardown(
		&mut self,
		teardown: super::Teardown<Self::Context>,
		context: &mut Self::Context,
	) {
		if let Ok(mut lock) = self.handle.write() {
			lock.add_teardown(teardown, context);
		} else {
			panic!("SubscriptionHandle's lock is poisoned!");
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Ok(mut lock) = self.handle.write() {
			lock.get_context_to_unsubscribe_on_drop()
		} else {
			panic!("SubscriptionHandle's lock is poisoned!");
		}
	}
}

impl<Subscription> Drop for SubscriptionHandle<Subscription>
where
	Subscription: TickableSubscription,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
