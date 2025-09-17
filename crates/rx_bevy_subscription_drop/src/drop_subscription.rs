use std::sync::{Arc, RwLock};

use rx_bevy_core::{
	DropContext, DropContextFromSubscription, InnerSubscription, SignalContext,
	SubscriptionCollection, SubscriptionLike, Teardown,
};

/// A DropSubscription is a type of Subscription Observables may use, it
/// requires the subscriptions SignalContext to be irrelevant during
/// unsubscription which is achieved by the [DropContext] trait that allows
/// creating this context out of the subscription itself
#[derive(Clone)]
pub struct DropSubscription<Context>
where
	Context: DropContext,
{
	inner: Arc<RwLock<InnerDropSubscription<Context>>>,
}

impl<Context> DropSubscription<Context>
where
	Context: DropContext,
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
	Context: DropContext,
{
	fn default() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::default())),
		}
	}
}

impl<Context> SignalContext for DropSubscription<Context>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for DropSubscription<Context>
where
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		let mut lock = self.inner.write().expect("to not be locked");
		lock.unsubscribe(context);
	}
}

impl<Context> SubscriptionCollection for DropSubscription<Context>
where
	Context: DropContext,
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
	Context: DropContext;

impl<Context> InnerDropSubscription<Context>
where
	Context: DropContext,
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

impl<Context> DropContextFromSubscription for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn get_unsubscribe_context(&mut self) -> Option<Self::Context> {
		Some(Context::get_context_for_drop())
	}
}

impl<Context> Default for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn default() -> Self {
		Self(InnerSubscription::default())
	}
}

impl<Context> SignalContext for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		self.0.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		self.0.unsubscribe(context);
	}
}

impl<Context> SubscriptionCollection for InnerDropSubscription<Context>
where
	Context: DropContext,
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
	Context: DropContext,
{
	fn drop(&mut self) {
		if let Some(mut context) = self.get_unsubscribe_context() {
			self.unsubscribe(&mut context);
		} else {
			self.unsubscribe(&mut Context::get_context_for_drop());
		}
	}
}
