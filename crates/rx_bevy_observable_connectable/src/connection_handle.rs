use rx_bevy_core::{
	SubscriptionLike, Teardown,
	context::{
		SubscriptionContext, WithSubscriptionContext, allocator::UnscheduledSubscriptionAllocator,
	},
};

/// Subscription that represents an active connection for a
/// [ConnectableObservable][crate::ConnectableObservable].
pub struct ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	handle: <<Subscription::Context as SubscriptionContext>::UnscheduledSubscriptionAllocator as UnscheduledSubscriptionAllocator>::UnscheduledHandle<Subscription>,
}

impl<Subscription> ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	pub fn new(subscription: Subscription, context: &mut Subscription::Context) -> Self {
		let handle =
			<<Subscription::Context as SubscriptionContext>::UnscheduledSubscriptionAllocator as UnscheduledSubscriptionAllocator>::allocate_unscheduled_subscription(
				subscription, context
			);
		Self { handle }
	}
}

impl<Subscription> Clone for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	fn clone(&self) -> Self {
		Self {
			handle: self.handle.clone(),
		}
	}
}

impl<Subscription> WithSubscriptionContext for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	type Context = Subscription::Context;
}

impl<Subscription> SubscriptionLike for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.handle.is_closed()
	}
	#[inline]
	fn unsubscribe(&mut self, context: &mut Self::Context) {
		self.handle.unsubscribe(context);
	}
	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.handle.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.handle.get_context_to_unsubscribe_on_drop()
	}
}

impl<Subscription> Drop for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
