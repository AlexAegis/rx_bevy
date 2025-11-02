use rx_core_traits::{
	SignalBound, SubscriptionContext, SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable,
	WithSubscriptionContext, allocator::ErasedDestinationAllocator,
};

/// This Subscription extends a shared subscriber into a clone-able subscription
/// To be a proper subscription it must also implement [Default] in order to be
/// used in contexts (combinator observables like [ZipObservable] and [CombineLatestObservable]) where multiple subscriptions has to be wrapped in one
pub struct MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	subscriber: Option<
		<Context::ErasedDestinationAllocator as ErasedDestinationAllocator>::Shared<In, InError>,
	>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	pub fn new(
		shared_subscriber: <Context::ErasedDestinationAllocator as ErasedDestinationAllocator>::Shared<In, InError>,
	) -> Self {
		Self {
			subscriber: Some(shared_subscriber),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<In, InError, Context> Default for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn default() -> Self {
		Self {
			subscriber: None,
			teardown: SubscriptionData::default(),
		}
	}
}

impl<In, InError, Context> Clone for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn clone(&self) -> Self {
		Self {
			subscriber: self.subscriber.clone(),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<In, InError, Context> WithSubscriptionContext for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<In, InError, Context> Tickable for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(subscriber) = &mut self.subscriber {
			subscriber.tick(tick, context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		self.teardown.is_closed()
			&& self
				.subscriber
				.as_ref()
				.map(|subscriber| subscriber.is_closed())
				.unwrap_or(true)
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		println!("multicast  sub unsub! is closed {}", self.is_closed());
		if !self.is_closed() {
			if let Some(mut subscriber) = self.subscriber.take() {
				subscriber.unsubscribe(context);
			}
			self.teardown.unsubscribe(context);
		}
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if let Some(subscriber) = &mut self.subscriber {
			subscriber.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}
}

impl<In, InError, Context> Drop for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		if !self.teardown.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.teardown.unsubscribe(&mut context);
		}
		// Does not unsubscribe the subscriber on drop as it is shared.
		// Only the teardown is unsubscribed which is local to the reference instance
	}
}
