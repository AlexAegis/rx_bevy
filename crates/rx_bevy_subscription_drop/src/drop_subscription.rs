use rx_bevy_core::{
	DropSafeSignalContext, SignalContext, SubscriptionData, SubscriptionLike, Teardown, Tick,
	Tickable, TickableSubscription, WithContext,
};

/// A DropSubscription is a type of Subscription Observables may use, it
/// requires the subscriptions SignalContext to be irrelevant during
/// unsubscription.
pub struct DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	subscription_data: SubscriptionData<Context>,
}

impl<Context> DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	pub fn new<S>(subscription: S) -> Self
	where
		S: TickableSubscription<Context = Context> + 'static + Send + Sync,
	{
		Self {
			subscription_data: SubscriptionData::new_from_resource(subscription.into()),
		}
	}
}

impl<Context> Default for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self {
			subscription_data: SubscriptionData::default(),
		}
	}
}

impl<Context> WithContext for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<Context> Tickable for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.subscription_data.tick(tick, context);
	}
}

impl<Context> SubscriptionLike for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		self.subscription_data.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			self.subscription_data.unsubscribe(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subscription_data.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<Context> Drop for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn drop(&mut self) {
		// While we require the context to be drop-safe, some contexts (like
		// the MockContext) may lie about its safety, so it's mandatory to still
		// check closed-ness before attempting an unsubscribe.
		// Not to mention that if the subscription is closed, it doesn't make
		// sense to trigger an unsubscription again on drop when one was already
		// done manually.
		if !self.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
