use std::sync::{Arc, RwLock, Weak};

use crate::{
	ObservableSubscription, SubscriptionLike, Teardown,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::{ScheduledSubscriptionHandle, WeakSubscriptionHandle},
	},
};
use short_type_name::short_type_name;

use super::ScheduledHeapSubscriptionHandle;

pub struct WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	subscription: Weak<RwLock<Subscription>>,
}

impl<Subscription> WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	pub(crate) fn new(subscription: &Arc<RwLock<Subscription>>) -> Self {
		Self {
			subscription: Arc::downgrade(subscription),
		}
	}
}

impl<Subscription> WithSubscriptionContext for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> WeakSubscriptionHandle for WeakHeapSubscriptionHandle<Subscription> where
	Subscription: SubscriptionLike + Send + Sync
{
}

impl<Subscription> Clone for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription: self.subscription.clone(),
		}
	}
}

impl<Subscription> SubscriptionLike for WeakHeapSubscriptionHandle<Subscription>
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

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed()
			&& let Some(subscription) = self.subscription.upgrade()
		{
			if let Ok(mut lock) = subscription.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed()
			&& let Some(subscription) = self.subscription.upgrade()
		{
			if let Ok(mut lock) = subscription.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Subscription> From<ScheduledHeapSubscriptionHandle<Subscription>>
	for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn from(mut subscription: ScheduledHeapSubscriptionHandle<Subscription>) -> Self {
		subscription.downgrade()
	}
}

impl<Subscription> Drop for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		// Must not do anything on drop as it's not owning the subscription it's
		// referring to.
	}
}
