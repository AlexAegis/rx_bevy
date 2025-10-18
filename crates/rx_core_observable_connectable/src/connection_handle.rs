use rx_core_traits::{
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
	pub fn new(
		subscription: Subscription,
		context: &mut <Subscription::Context as SubscriptionContext>::Item<'_, '_>,
	) -> Self {
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
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		self.handle.unsubscribe(context);
	}
	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.handle.add_teardown(teardown, context);
	}
}

impl<Subscription> Drop for ConnectionHandle<Subscription>
where
	Subscription: 'static + SubscriptionLike + Send + Sync,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = Subscription::Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
