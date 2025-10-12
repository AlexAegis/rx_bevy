use std::sync::{Arc, RwLock, Weak};

use crate::{
	ArcSubscriptionHandle, ObservableSubscription, ScheduledSubscriptionHandle,
	SubscriptionContext, SubscriptionLike, Teardown, WeakSubscriptionHandle,
	WithSubscriptionContext,
};
use short_type_name::short_type_name;

pub struct WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	subscription: Weak<RwLock<Subscription>>,
}

impl<Subscription> WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	pub(crate) fn new(subscription: &Arc<RwLock<Subscription>>) -> Self {
		Self {
			subscription: Arc::downgrade(subscription),
		}
	}
}

impl<Subscription> WithSubscriptionContext for WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> WeakSubscriptionHandle for WeakArcSubscriptionHandle<Subscription> where
	Subscription: SubscriptionLike + Send + Sync
{
}

impl<Subscription> Clone for WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription: self.subscription.clone(),
		}
	}
}

impl<Subscription> SubscriptionLike for WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn is_closed(&self) -> bool {
		if let Some(subscription) = self.subscription.upgrade() {
			if let Ok(lock) = subscription.read() {
				lock.is_closed()
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
				true
			}
		} else {
			// It was dropped already
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(subscription) = self.subscription.upgrade() {
				if let Ok(mut lock) = subscription.write() {
					lock.unsubscribe(context);
				} else {
					println!("Poisoned destination lock: {}", short_type_name::<Self>());
				}
			}
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(subscription) = self.subscription.upgrade() {
				if let Ok(mut lock) = subscription.write() {
					lock.add_teardown(teardown, context);
				} else {
					println!("Poisoned destination lock: {}", short_type_name::<Self>());
				}
			}
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		if let Some(subscription) = self.subscription.upgrade() {
			if let Ok(mut lock) = subscription.write() {
				lock.get_context_to_unsubscribe_on_drop()
			} else {
				panic!(
					"Context can't be acquired in a {} as the destination RwLock is poisoned!",
					short_type_name::<Self>()
				)
			}
		} else {
			Self::Context::create_context_to_unsubscribe_on_drop()
		}
	}
}

impl<Subscription> From<ArcSubscriptionHandle<Subscription>>
	for WeakArcSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn from(mut subscription: ArcSubscriptionHandle<Subscription>) -> Self {
		subscription.downgrade()
	}
}

impl<Subscription> Drop for WeakArcSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		// Must not do anything on drop as it's not owning the subscription it's
		// referring to.
	}
}
