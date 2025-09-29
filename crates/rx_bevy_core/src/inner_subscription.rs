use short_type_name::short_type_name;

use crate::{DropContext, SignalContext, SubscriptionLike, Teardown};
use std::fmt::Debug;

pub struct InnerSubscription<Context>
where
	Context: DropContext,
{
	is_closed: bool,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> Debug for InnerSubscription<Context>
where
	Context: DropContext,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {}, finalizers: {} }}",
			short_type_name::<Self>(),
			self.is_closed,
			self.finalizers.len()
		))
	}
}

impl<Context> InnerSubscription<Context>
where
	Context: DropContext,
{
	pub fn new(subscription: impl Into<Teardown<Context>>) -> Self {
		let teardown: Teardown<Context> = subscription.into();

		if let Some(teardown_fn) = teardown.take() {
			Self {
				is_closed: false,
				finalizers: vec![teardown_fn],
			}
		} else {
			Self {
				is_closed: false,
				finalizers: Vec::default(),
			}
		}
	}

	pub fn new_fn<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self::new(Teardown::<Context>::new(f))
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
	#[inline]
	fn is_closed(&self) -> bool {
		self.is_closed
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		if !self.is_closed() {
			self.is_closed = true;

			for teardown in self.finalizers.drain(..) {
				(teardown)(context);
			}
		}
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		if self.is_closed() {
			// If this subscription is already closed, the added one is unsubscribed immediately
			teardown.call(context);
		} else if let Some(teardown_fn) = teardown.take() {
			self.finalizers.push(teardown_fn);
		}
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		// May or may not panic, depending on the context used.
		// If you want to make sure it doesn't panic, use DropSafe contexts!
		// If you do need to use DropUnsafe contexts, make sure you unsubscribe
		// it before letting it go out of scope and drop!
		Context::get_context_for_drop()
	}
}
