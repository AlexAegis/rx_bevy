use rx_bevy_core::{
	ErasedArcSubscriber, SignalBound, SignalContext, SubscriptionData, SubscriptionLike, Teardown,
	Tick, Tickable, WithContext,
};

/// This Subscription extends a shared subscriber into a clone-able subscription
/// To be a proper subscription it must also implement [Default] in order to be
/// used in contexts (combinator observables like [ZipObservable] and [CombineLatestObservable]) where multiple subscriptions has to be wrapped in one
pub struct MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	subscriber: Option<ErasedArcSubscriber<In, InError, Context>>,
	teardown: SubscriptionData<Context>,
}

impl<In, InError, Context> MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	pub fn new(shared_subscriber: ErasedArcSubscriber<In, InError, Context>) -> Self {
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
	Context: SignalContext,
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
	Context: SignalContext,
{
	fn clone(&self) -> Self {
		Self {
			subscriber: self.subscriber.clone(),
			teardown: SubscriptionData::default(),
		}
	}
}

impl<In, InError, Context> WithContext for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	type Context = Context;
}

impl<In, InError, Context> Tickable for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		if let Some(subscriber) = &mut self.subscriber {
			subscriber.tick(tick, context);
		}
	}
}

impl<In, InError, Context> SubscriptionLike for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		self.subscriber
			.as_ref()
			.map(|s| s.is_closed())
			.unwrap_or(true)
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			if let Some(mut subscriber) = self.subscriber.take() {
				subscriber.unsubscribe(context);
			}
			self.teardown.unsubscribe(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if let Some(subscriber) = &mut self.subscriber {
			subscriber.add_teardown(teardown, context);
		} else {
			teardown.execute(context);
		}
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<In, InError, Context> Drop for MulticastSubscription<In, InError, Context>
where
	In: SignalBound + Clone,
	InError: SignalBound + Clone,
	Context: SignalContext,
{
	fn drop(&mut self) {
		if !self.teardown.is_closed() {
			let mut context = self.teardown.get_context_to_unsubscribe_on_drop();
			self.teardown.unsubscribe(&mut context);
		}
		// Does not unsubscribe the subscriber on drop as it is shared.
		// Only the teardown is unsubscribed which is local to the reference instance
	}
}
