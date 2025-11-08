// TODO: Check import groups, std -> dependencies -> crate -> super, similar to the nightly rustfmt option https://rust-lang.github.io/rustfmt/?version=v1.8.0&search=#group_imports
use std::sync::{Arc, RwLock};

use disqualified::ShortName;

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
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	subscription: Arc<RwLock<Subscription>>,
}

impl<Subscription> UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
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
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	type WeakHandle = WeakHeapSubscriptionHandle<Subscription>;

	fn downgrade(&mut self) -> Self::WeakHandle {
		WeakHeapSubscriptionHandle::new(&self.subscription)
	}
}

impl<Subscription> WithSubscriptionContext for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> Clone for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			subscription: self.subscription.clone(),
		}
	}
}

impl<Subscription> SubscriptionLike for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn is_closed(&self) -> bool {
		if let Ok(lock) = self.subscription.read() {
			lock.is_closed()
		} else {
			println!("Poisoned destination lock: {}", ShortName::of::<Self>());
			true
		}
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.subscription.write() {
				lock.unsubscribe(context);
			} else {
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
				// TODO: research poisoned lock recovery, maybe it should panic?
			}
		}
	}
}

impl<Subscription> TeardownCollection for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			if let Ok(mut lock) = self.subscription.write() {
				lock.add_teardown(teardown, context);
			} else {
				println!("Poisoned destination lock: {}", ShortName::of::<Self>());
			}
		}
	}
}

impl<Subscription> Drop for UnscheduledHeapSubscriptionHandle<Subscription>
where
	Subscription: SubscriptionWithTeardown + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
