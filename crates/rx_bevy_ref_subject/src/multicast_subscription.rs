use rx_bevy_core::{
	DropContext, ErasedArcSubscriber, InnerSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown,
};

pub struct MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	subscriber: Option<ErasedArcSubscriber<In, InError, Context>>,
	inner: InnerSubscription<Context>,
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
			inner: InnerSubscription::default(),
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
			inner: InnerSubscription::default(),
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
		if let Some(mut subscriber) = self.subscriber.take() {
			subscriber.unsubscribe(context);
		}
		self.inner.unsubscribe(context);
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<In, InError, Context> SubscriptionCollection for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.inner.add::<S, T>(subscription, context);
	}
}

impl<In, InError, Context> Drop for MulticastSubscription<In, InError, Context>
where
	In: 'static + Clone,
	InError: 'static + Clone,
	Context: DropContext,
{
	fn drop(&mut self) {
		// Does not unsubscribe on drop as it is shared.
	}
}
