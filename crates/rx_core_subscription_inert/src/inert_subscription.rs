use rx_core_traits::{
	SubscriptionContext, SubscriptionLike, SubscriptionScheduled, Teardown, TeardownCollection,
	Tick, Tickable, WithSubscriptionContext,
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
	Context: SubscriptionContext,
{
	tickable: Box<dyn Tickable<Context = Context> + Send + Sync>,
	// TODO: Check every PhantomData for variance
}

impl<Context> InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	pub fn new(
		mut destination: impl SubscriptionScheduled<Context = Context> + 'static + Send + Sync,
		context: &mut Context::Item<'_, '_>,
	) -> Self {
		// Immediately unsubscribes if it's not already closed.
		if !destination.is_closed() {
			destination.unsubscribe(context);
		}

		Self {
			tickable: Box::new(destination),
		}
	}
}

impl<Context> WithSubscriptionContext for InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	type Context = Context;
}

impl<Context> Tickable for InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn tick(
		&mut self,
		tick: Tick,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		self.tickable.tick(tick, context);
	}
}

impl<Context> SubscriptionLike for InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		// Does not need to do anything on unsubscribe
	}
}

impl<Context> TeardownCollection for InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		// The added teardown is executed immediately as this subscription is always closed.
		teardown.execute(context);
	}
}

impl<Context> Drop for InertSubscription<Context>
where
	Context: SubscriptionContext,
{
	fn drop(&mut self) {
		// Does not need to do anything on drop, as it contains nothing.
	}
}
