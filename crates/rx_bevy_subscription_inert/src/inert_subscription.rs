use rx_bevy_core::{
	SignalContext, SubscriptionLike, Teardown, Tick, Tickable, TickableSubscription, WithContext,
};

/// A [InertSubscription] is a permanently closed [Subscription] that immediately
/// runs any [Teardown] you may add into it.
/// It is used for [Observable]s that emit all their values, complete and
/// unsubscribe immediately on subscribe.
/// This aspect lets us safely ignore the drop-safety of the context used, as
/// subscriptions made with drop-unsafe contexts can (obviously) be dropped once
/// they are unsubscribed, and that is guaranteed here.
pub struct InertSubscription<Context>
where
	Context: SignalContext,
{
	tickable: Box<dyn Tickable<Context = Context> + Send + Sync>,
	// TODO: Check every PhantomData for variance
}

impl<Context> InertSubscription<Context>
where
	Context: SignalContext,
{
	pub fn new(
		mut destination: impl TickableSubscription<Context = Context> + 'static + Send + Sync,
		context: &mut Context,
	) -> Self {
		destination.unsubscribe(context);

		Self {
			tickable: Box::new(destination),
		}
	}
}

impl<Context> WithContext for InertSubscription<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> Tickable for InertSubscription<Context>
where
	Context: SignalContext,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.tickable.tick(tick, context);
	}
}

impl<Context> SubscriptionLike for InertSubscription<Context>
where
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		// Does not need to do anything on unsubscribe
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		// The added teardown is executed immediately as this subscription is always closed.
		teardown.execute(context);
	}
}

impl<Context> Drop for InertSubscription<Context>
where
	Context: SignalContext,
{
	fn drop(&mut self) {
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
