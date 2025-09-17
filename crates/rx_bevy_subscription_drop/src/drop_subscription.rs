use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropContext, DropSafeSignalContext, InnerSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown,
};

/// A DropSubscription is a type of Subscription Observables may use, it
/// requires the subscriptions SignalContext to be irrelevant during
/// unsubscription.
#[derive(Clone)]
pub struct DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	inner: Arc<RwLock<InnerDropSubscription<Context>>>,
}

impl<Context> DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	pub fn new<S, T>(subscription: T) -> Self
	where
		S: SubscriptionLike<Context = Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::new(subscription))),
		}
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Self, Context>::new(f))
	}
}

impl<Context> Default for DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::default())),
		}
	}
}

impl<Context> SignalContext for DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		let mut lock = self.inner.write().expect("to not be locked");
		lock.unsubscribe(context);
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<Context> SubscriptionCollection for DropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let mut lock = self.inner.write().expect("to not be locked");
		lock.add(subscription, context);
	}
}

pub struct InnerDropSubscription<Context>(InnerSubscription<Context>)
where
	Context: DropContext<DropSafety = DropSafeSignalContext>;

impl<Context> InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	pub fn new<S, T>(subscription: T) -> Self
	where
		S: SubscriptionLike<Context = Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		Self(InnerSubscription::new(subscription))
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Self, Context>::new(f))
	}
}

impl<Context> Default for InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn default() -> Self {
		Self(InnerSubscription::default())
	}
}

impl<Context> SignalContext for InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
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
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		self.0.get_unsubscribe_context()
	}
}

impl<Context> SubscriptionCollection for InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.0.add(subscription, context);
	}
}

impl<Context> Drop for InnerDropSubscription<Context>
where
	Context: DropContext<DropSafety = DropSafeSignalContext>,
{
	fn drop(&mut self) {
		// This is safe to do, because we require the context to be drop safe!
		let mut context = self.get_unsubscribe_context();
		self.unsubscribe(&mut context);
	}
}
