use std::sync::{Arc, RwLock};

use smallvec::SmallVec;

use crate::{
	DropContext, DropContextFromSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown,
};

// TODO: Move this to its own crate
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

impl<Context> DropSubscription<Context>
where
	Context: DropContext,
{
	pub fn new(finalizer: impl Into<Teardown<Context>>) -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::new(finalizer))),
		}
	}

	pub fn new_empty() -> Self {
		Self {
			inner: Arc::new(RwLock::new(InnerDropSubscription::new_empty())),
		}
	}

	pub fn add(&mut self, finalizer: impl Into<Teardown<Context>>, context: &mut Context) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call(context);
		} else {
			self.inner
				.write()
				.expect("not locked")
				.finalizers
				.push(finalizer.into());
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
		self.inner.read().expect("to not be locked").is_closed
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
	fn add(&mut self, subscription: impl Into<Teardown<Context>>, context: &mut Context) {
		let mut lock = self.inner.write().expect("to not be locked");

		lock.add_finalizer(subscription, context);
	}
}

pub struct InnerDropSubscription<Context>
where
	Context: DropContext,
{
	is_closed: bool,
	finalizers: SmallVec<[Teardown<Context>; 1]>,
}

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
		Self {
			finalizers: SmallVec::new(),
			is_closed: false,
		}
	}
}

impl<Context> SignalContext for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<Context> InnerDropSubscription<Context>
where
	Context: DropContext,
{
	pub fn new(finalizer: impl Into<Teardown<Context>>) -> Self {
		let teardown = finalizer.into();
		let is_closed = matches!(&teardown, Teardown::Sub(sub) if sub.is_closed());

		Self {
			is_closed,
			finalizers: if is_closed {
				SmallVec::new()
			} else {
				smallvec::smallvec![teardown]
			},
		}
	}

	pub fn new_empty() -> Self {
		Self {
			is_closed: false,
			finalizers: SmallVec::new(),
		}
	}

	pub fn add_finalizer(
		&mut self,
		finalizer: impl Into<Teardown<Context>>,
		context: &mut Context,
	) {
		if self.is_closed() {
			// If the subscription is already closed, the finalizer is called immediately
			finalizer.into().call(context);
		} else {
			self.finalizers.push(finalizer.into());
		}
	}
}

impl<Context> SubscriptionLike for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.is_closed {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				teardown.call(context);
			}
		}
	}
}

impl<Context> SubscriptionCollection for InnerDropSubscription<Context>
where
	Context: DropContext,
{
	fn add(&mut self, subscription: impl Into<Teardown<Context>>, context: &mut Context) {
		self.add_finalizer(subscription, context);
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
