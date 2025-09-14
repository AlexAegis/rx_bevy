use std::marker::PhantomData;

use crate::SubscriptionLike;

pub struct Teardown<S, Context> {
	teardown_fn: Option<Box<dyn FnOnce(&mut Context)>>,
	_phantom_data: PhantomData<S>,
}

impl<S, Context> Teardown<S, Context> {
	pub fn new<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context),
	{
		Self {
			teardown_fn: Some(Box::new(f)),
			_phantom_data: PhantomData,
		}
	}

	/// Consumes the [Teardown] without executing it, returning the stored
	/// function if it wasn't already closed.
	/// Used when the stored function is moved to somewhere else, like into a
	/// subscription.
	#[inline]
	pub fn take(mut self) -> Option<Box<dyn FnOnce(&mut Context)>> {
		self.teardown_fn.take()
	}

	/// Immediately consumes and calls the teardown.
	/// Useful if you just want to execute it and not store it for later.
	#[inline]
	pub fn call(mut self, context: &mut Context) {
		if let Some(teardown) = self.teardown_fn.take() {
			(teardown)(context);
		}
	}

	#[inline]
	pub fn is_closed(&self) -> bool {
		self.teardown_fn.is_none()
	}
}

impl<S, Context> Default for Teardown<S, Context> {
	fn default() -> Self {
		Self {
			teardown_fn: None,
			_phantom_data: PhantomData,
		}
	}
}

/// Exposes and respects the original subscriptions closed-ness by storing it
/// in an option.
/// TODO: Make sure that dropping the value here when it's closed isn't a problem
impl<S> From<S> for Teardown<S, S::Context>
where
	S: 'static + SubscriptionLike,
{
	fn from(mut value: S) -> Self {
		Self {
			teardown_fn: if value.is_closed() {
				None
			} else {
				Some(Box::new(move |context| value.unsubscribe(context)))
			},
			_phantom_data: PhantomData,
		}
	}
}
