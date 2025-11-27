use std::sync::{Arc, RwLock, Weak};

use crate::{
	SubscriptionLike, SubscriptionScheduled, SubscriptionWithTeardown, Teardown,
	TeardownCollection,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::{ScheduledSubscriptionHandle, WeakSubscriptionHandle},
	},
};

use super::ScheduledHeapSubscriptionHandle;

pub struct WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	subscription: Weak<RwLock<Subscription>>,
}

impl<Subscription> WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	pub(crate) fn new(subscription: &Arc<RwLock<Subscription>>) -> Self {
		Self {
			subscription: Arc::downgrade(subscription),
		}
	}
}

impl<Subscription> WithSubscriptionContext for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> WeakSubscriptionHandle for WeakHeapSubscriptionHandle<Subscription> where
	Subscription: SubscriptionWithTeardown + Send + Sync
{
}

impl<Subscription> Clone for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription: self.subscription.clone(),
		}
	}
}

impl<Subscription> SubscriptionLike for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn is_closed(&self) -> bool {
		if let Some(subscription) = self.subscription.upgrade() {
			subscription.is_closed()
		} else {
			// It was dropped already
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed()
			&& let Some(mut subscription) = self.subscription.upgrade()
		{
			subscription.unsubscribe(context);
		}
	}
}

impl<Subscription> TeardownCollection for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed()
			&& let Some(mut subscription) = self.subscription.upgrade()
		{
			subscription.add_teardown(teardown, context);
		}
	}
}

impl<Subscription> From<ScheduledHeapSubscriptionHandle<Subscription>>
	for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionScheduled + Send + Sync,
{
	fn from(mut subscription: ScheduledHeapSubscriptionHandle<Subscription>) -> Self {
		subscription.downgrade()
	}
}

impl<Subscription> Drop for WeakHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn drop(&mut self) {
		// Must not do anything on drop as it's not owning the subscription it's
		// referring to.
	}
}
