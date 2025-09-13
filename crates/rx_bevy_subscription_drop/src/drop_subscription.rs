use std::sync::{Arc, RwLock};

use rx_bevy_core::{InnerSubscription, SignalContext, SubscriptionCollection, SubscriptionLike};

use crate::{DropContext, DropContextFromSubscription};

/// A DropSubscription is a type of Subscription Observables may use, it
/// requires the subscriptions SignalContext to be irrelevant during
/// unsubscription which is achieved by the [DropContext] trait that allows
/// creating this context out of the subscription itself
#[derive(Clone)]
pub struct DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	inner: Arc<RwLock<InnerDropSubscription<'c, Context>>>,
}

impl<'c, Context> DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	pub fn new<S>(subscription: S, context: &mut Context) -> Self
	where
		S: 'c + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		let mut inner = InnerDropSubscription::default();
		inner.add(subscription, context);
		Self {
			inner: Arc::new(RwLock::new(inner)),
		}
	}

	pub fn new_from<S>(subscription: impl Into<S>, context: &mut Context) -> Self
	where
		S: 'c + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		Self::new(subscription.into(), context)
	}
}

impl<'c, Context> Default for DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn default() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::default())),
		}
	}
}

impl<'c, Context> SignalContext for DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	type Context = Context;
}

impl<'c, Context> SubscriptionLike for DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn is_closed(&self) -> bool {
		self.inner.read().expect("to not be locked").is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Self::Context) {
		let mut lock = self.inner.write().expect("to not be locked");
		lock.unsubscribe(context);
	}
}

impl<'c, Context> SubscriptionCollection<'c> for DropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'c + SubscriptionLike<Context = <Self as SignalContext>::Context>,
	{
		let mut lock = self.inner.write().expect("to not be locked");
		lock.add(subscription, context);
	}
}

pub struct InnerDropSubscription<'c, Context: 'c>(InnerSubscription<'c, Context>)
where
	Context: DropContext;

impl<'c, Context> DropContextFromSubscription for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<'c, Context> Default for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn default() -> Self {
		Self(InnerSubscription::default())
	}
}

impl<'c, Context> SignalContext for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	type Context = Context;
}

impl<'c, Context> SubscriptionLike for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn is_closed(&self) -> bool {
		self.0.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		self.0.unsubscribe(context);
	}
}

impl<'c, Context> SubscriptionCollection<'c> for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn add<S>(&mut self, subscription: S, context: &mut Self::Context)
	where
		S: 'c + SubscriptionLike<Context = Self::Context>,
	{
		self.0.add(subscription, context);
	}
}

impl<'c, Context> Drop for InnerDropSubscription<'c, Context>
where
	Context: DropContext + 'c,
{
	fn drop(&mut self) {
		let mut context = self.get_unsubscribe_context();
		self.unsubscribe(&mut context);
	}
}
