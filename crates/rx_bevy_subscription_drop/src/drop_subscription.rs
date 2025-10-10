use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropSafeSignalContext, SignalContext, SubscriptionData, SubscriptionLike, Teardown, Tick,
	Tickable, TickableSubscription, WithContext,
};

/// A DropSubscription is a type of Subscription Observables may use, it
/// requires the subscriptions SignalContext to be irrelevant during
/// unsubscription.
#[derive(Clone)]
pub struct DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	inner: Arc<RwLock<InnerDropSubscription<Context>>>,
}

impl<Context> DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	pub fn new<S>(subscription: S) -> Self
	where
		S: TickableSubscription<Context = Context> + 'static,
	{
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription(
				SubscriptionData::new_from_resource(subscription.into()),
			))),
		}
	}
}

impl<Context> Default for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::default())),
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
		if let Ok(mut lock) = self.inner.write() {
			lock.0.tick(tick, context);
		}
	}
}

impl<Context> SubscriptionLike for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").0.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.inner.write().expect("to not be locked");
			lock.0.unsubscribe(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		let mut lock = self.inner.write().expect("to not be locked");
		lock.0.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

pub struct InnerDropSubscription<Context>(SubscriptionData<Context>)
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>;

impl<Context> Default for InnerDropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self(SubscriptionData::default())
	}
}

impl<Context> Drop for InnerDropSubscription<Context>
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
		if !self.0.is_closed() {
			let mut context = Context::create_context_to_unsubscribe_on_drop();
			self.0.unsubscribe(&mut context);
		}
	}
}
