use std::sync::{Arc, RwLock};

use short_type_name::short_type_name;

use crate::{
	ObservableSubscription, SubscriptionLike, Teardown, Tick, Tickable,
	context::{
		SubscriptionContext, WithSubscriptionContext,
		allocator::handle::ScheduledSubscriptionHandle,
	},
};

use super::{UnscheduledHeapSubscriptionHandle, WeakHeapSubscriptionHandle};

pub struct ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	subscription: Arc<RwLock<Subscription>>,
}

impl<Subscription> ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	pub fn new(subscription: Subscription) -> Self {
		Self {
			subscription: Arc::new(RwLock::new(subscription)),
		}
	}
}

impl<Subscription> ScheduledSubscriptionHandle for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
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
	Subscription: ObservableSubscription + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Tickable for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if let Ok(mut lock) = self.subscription.write() {
			lock.tick(tick, context);
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
		}
	}
}

impl<Subscription> SubscriptionLike for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.subscription.read() {
			lock.is_closed()
		} else {
			println!("Poisoned destination lock: {}", short_type_name::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.subscription.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
				// TODO: research poisoned lock recovery, maybe it should panic?
			}
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.subscription.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", short_type_name::<Self>());
			}
		}
	}
}

impl<Subscription> From<Subscription> for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn from(subscription: Subscription) -> Self {
		Self::new(subscription)
	}
}

impl<Subscription> Drop for ScheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: ObservableSubscription + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
