use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropSafeSignalContext, SignalContext, Subscription, SubscriptionLike, Teardown, WithContext,
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
	pub fn new<T>(subscription: T) -> Self
	where
		T: Into<Teardown<Context>>,
	{
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::new(subscription))),
		}
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Context>::new(f))
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

impl<Context> SubscriptionLike for DropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		if !self.is_closed() {
			let mut lock = self.inner.write().expect("to not be locked");
			lock.unsubscribe(context);
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		let mut lock = self.inner.write().expect("to not be locked");
		lock.add_teardown(teardown, context);
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

pub struct InnerDropSubscription<Context>(Subscription<Context>)
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>;

impl<Context> InnerDropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	pub fn new<T>(subscription: T) -> Self
	where
		T: Into<Teardown<Context>>,
	{
		Self(Subscription::new(subscription))
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Context>::new(f))
	}
}

impl<Context> Default for InnerDropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self(Subscription::default())
	}
}

impl<Context> WithContext for InnerDropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerDropSubscription<Context>
where
	Context: SignalContext<DropSafety = DropSafeSignalContext>,
{
	#[inline]
	fn is_closed(&self) -> bool {
		self.0.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Context) {
		self.0.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.0.add_teardown(teardown, context);
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		self.0.get_context_to_unsubscribe_on_drop()
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
		if !self.is_closed() {
			let mut context = self.get_context_to_unsubscribe_on_drop();
			self.unsubscribe(&mut context);
		}
	}
}
