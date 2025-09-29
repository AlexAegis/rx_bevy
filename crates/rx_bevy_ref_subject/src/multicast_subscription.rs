use rx_bevy_core::{
	DropContext, ErasedArcSubscriber, InnerSubscription, SignalContext, SubscriptionLike, Teardown,
};

/// This Subscription extends a shared subscriber into a clone-able subscription
/// To be a proper subscription it must also implement [Default] in order to be
/// used in contexts (combinator observables like [ZipObservable] and [CombineLatestObservable]) where multiple subscriptions has to be wrapped in one
pub struct MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	subscriber: Option<ErasedArcSubscriber<In, InError, Context>>,
	teardown: InnerSubscription<Context>,
}

impl<In, InError, Context> MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	pub fn new(shared_subscriber: ErasedArcSubscriber<In, InError, Context>) -> Self {
		Self {
			subscriber: Some(shared_subscriber),
			teardown: InnerSubscription::default(),
		}
	}
}

impl<In, InError, Context> Default for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn default() -> Self {
		Self {
			subscriber: None,
			teardown: InnerSubscription::default(),
		}
	}
}

impl<In, InError, Context> Clone for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn clone(&self) -> Self {
		Self {
			subscriber: self.subscriber.clone(),
			teardown: InnerSubscription::default(),
		}
	}
}

impl<In, InError, Context> SignalContext for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	type Context = Context;
}

impl<In, InError, Context> SubscriptionLike for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
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
			teardown.call(context);
		}
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<In, InError, Context> Drop for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn drop(&mut self) {
		if !self.teardown.is_closed() {
			let mut context = self.teardown.get_unsubscribe_context();
			self.teardown.unsubscribe(&mut context);
		}
		// Does not unsubscribe the subscriber on drop as it is shared.
		// Only the teardown is unsubscribed which is local to the reference instance
	}
}
