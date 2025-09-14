use std::sync::{Arc, RwLock};

use rx_bevy_core::{InnerSubscription, SignalContext, SubscriptionCollection, SubscriptionLike};

use crate::{DropContext, DropContextFromSubscription};

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
	pub fn new<S>(subscription: S, context: &mut Context) -> Self
	where
		S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		let mut inner = InnerDropSubscription::default();
		inner.add(subscription, context);
		Self {
			inner: Arc::new(RwLock::new(inner)),
		}
	}

	pub fn new_from<S>(subscription: impl Into<S>, context: &mut Context) -> Self
	where
		S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		Self::new(subscription.into(), context)
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
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'static + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		let mut lock = self.inner.write().expect("to not be locked");
		lock.add(subscription, context);
	}
}

pub struct InnerDropSubscription<Context>(InnerSubscription<Context>)
where
	Context: DropContext;

impl<Context> DropContextFromSubscription for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
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
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'static + SubscriptionLike<Context = Self::Context>,
	{
		self.0.add(subscription, context);
	}
}

impl<Context> Drop for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn drop(&mut self) {
		let mut context = self.get_unsubscribe_context();
		self.unsubscribe(&mut context);
	}
}
