use short_type_name::short_type_name;

use crate::{SignalContext, SubscriptionLike, Teardown, WithContext};
use std::fmt::Debug;

/// The base subscription implementation commonly used by other subscription
/// implementations.
///
/// This struct is just a collection of teardown closures, stored as the
/// closure itself.
///
/// This collection of closures represent the resources held by the
/// subscription. To release the resources the subscription must be unsubscribed
/// upon which the collection is drained, and the closures are called,
/// effectively dropping everything held by the subscription before the
/// subscription itself is dropped.
pub struct Subscription<Context>
where
	Context: SignalContext,
{
	is_closed: bool,
	finalizers: Vec<Box<dyn FnOnce(&mut Context)>>,
}

impl<Context> Subscription<Context>
where
	Context: SignalContext,
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

impl<Context> Default for Subscription<Context>
where
	Context: SignalContext,
{
	fn default() -> Self {
		Self {
			finalizers: Vec::new(),
			is_closed: false,
		}
	}
}

impl<Context> WithContext for Subscription<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for Subscription<Context>
where
	Context: SignalContext,
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
			// If this subscription is already closed, the newly added teardown
			// is immediately executed.
			teardown.execute(context);
		} else if let Some(teardown_fn) = teardown.take() {
			self.finalizers.push(teardown_fn);
		}
	}

	#[inline]
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		// May or may not panic, depending on the context used.
		// If you want to make sure it doesn't panic, use DropSafe contexts!
		// If you do need to use DropUnsafe contexts, make sure you unsubscribe
		// it before letting it go out of scope and drop!
		Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<Context> Debug for Subscription<Context>
where
	Context: SignalContext,
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
