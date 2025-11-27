use std::sync::{Arc, RwLock};

use crate::{
	SubscriptionLike, SubscriptionWithTeardown, Teardown, TeardownCollection,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::UnscheduledSubscriptionHandle,
	},
};

use super::WeakHeapSubscriptionHandle;

pub struct UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	subscription: Arc<RwLock<Subscription>>,
}

impl<Subscription> UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	pub fn new_from_handle_ref(handle_ref: &Arc<RwLock<Subscription>>) -> Self {
		Self {
			subscription: handle_ref.clone(),
		}
	}

	pub fn new(subscription: Subscription) -> Self {
		Self {
			subscription: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> UnscheduledSubscriptionHandle for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	type WeakHandle = WeakHeapSubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakHeapSubscriptionHandle::new(&self.subscription)
	}
}

impl<Subscription> WithSubscriptionContext for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Clone for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription: self.subscription.clone(),
		}
	}
}

impl<Subscription> SubscriptionLike for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.subscription.unsubscribe(context);
	}
}

impl<Subscription> TeardownCollection for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subscription.add_teardown(teardown, context);
	}
}

impl<Subscription> Drop for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionWithTeardown + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
