use std::sync::{Arc, RwLock};

use crate::{
	SubscriptionLike, SubscriptionScheduled, Teardown, TeardownCollection, Tick, Tickable,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::ScheduledSubscriptionHandle,
	},
};

use super::{UnscheduledHeapSubscriptionHandle, WeakHeapSubscriptionHandle};

pub struct ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	subscription: Arc<RwLock<Subscription>>,
}

impl<Subscription> ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	pub fn new(subscription: Subscription) -> Self {
		Self {
			subscription: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> ScheduledSubscriptionHandle for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	type UnscheduledHandle = UnscheduledHeapSubscriptionHandle<Subscription>;
	type WeakHandle = WeakHeapSubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakHeapSubscriptionHandle::new(&self.subscription)
	}

	fn clone(&self) -> Self::UnscheduledHandle {
		UnscheduledHeapSubscriptionHandle::new_from_handle_ref(&self.subscription)
	}
}

impl<Subscription> WithSubscriptionContext for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Tickable for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subscription.tick(tick, context);
	}
}

impl<Subscription> SubscriptionLike for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.subscription.unsubscribe(context);
	}
}

impl<Subscription> TeardownCollection for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.subscription.add_teardown(teardown, context);
	}
}

impl<Subscription> From<Subscription> for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	fn from(subscription: Subscription) -> Self {
		Self::new(subscription)
	}
}

impl<Subscription> Drop for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: 'static + SubscriptionScheduled + Send + Sync,
{
	#[track_caller]
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
