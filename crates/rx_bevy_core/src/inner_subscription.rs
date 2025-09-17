use crate::{DropContext, SignalContext, SubscriptionCollection, SubscriptionLike, Teardown};

pub struct InnerSubscription<Context>
where
	Context: DropContext,
{
	is_closed: bool,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> InnerSubscription<Context>
where
	Context: DropContext,
{
	pub fn new<S, T>(subscription: T) -> Self
	where
		S: SubscriptionLike<Context = Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();

		if let Some(teardown_fn) = teardown.take() {
			Self {
				is_closed: false,
				finalizers: vec![teardown_fn],
			}
		} else {
			Self::default()
		}
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Self, Context>::new(f))
	}
}

impl<Context> Default for InnerSubscription<Context>
where
	Context: DropContext,
{
	fn default() -> Self {
		Self {
			finalizers: Vec::new(),
			is_closed: false,
		}
	}
}

impl<Context> SignalContext for InnerSubscription<Context>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InnerSubscription<Context>
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
				(teardown)(context);
			}
		}
	}

	fn get_unsubscribe_context(&mut self) -> Option<Self::Context> {
		Some(Context::get_context_for_drop())
	}
}

impl<Context> SubscriptionCollection for InnerSubscription<Context>
where
	Context: DropContext,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		if self.is_closed() {
			// If this subscription is already closed, the added one is unsubscribed immediately
			teardown.call(context);
		} else {
			if let Some(teardown_fn) = teardown.take() {
				self.finalizers.push(teardown_fn);
			}
		}
	}
}
