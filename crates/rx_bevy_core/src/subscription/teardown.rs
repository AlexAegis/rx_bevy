use std::fmt::Debug;

use crate::SubscriptionLike;

/// A teardown is a closure which owns resources, by the nature of them being
/// moved into said closure. The closure itself is responsible for releasing
/// these resources.
///
/// For example if this resource was a subscription, the closure looks like this:
///
/// ```rs
/// move |context| subscription.unsubscribe(context)
/// ```
///
/// Just like subscriptions, a teardown once closed cannot be opened again.
///
/// [Teardown] intentionally does not implement [SubscriptionLike] to facilitate
/// the [SubscriptionCollection][crate::SubscriptionCollection] trait which
/// uses [Teardown] as the base type of operation. Allowing generic functions
/// where you can add anything that is `Into<Teardown>` such as Subscriptions.
pub struct Teardown<Context> {
	teardown_fn: Option<Box<dyn FnOnce(&mut Context) + Send + Sync>>,
}

impl<Context> Debug for Teardown<Context> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{} {{ is_closed: {} }}",
			short_type_name::short_type_name::<Self>(),
			self.is_closed(),
		))
	}
}

impl<Context> Teardown<Context> {
	pub fn new<F>(f: F) -> Self
	where
		F: 'static + FnOnce(&mut Context) + Send + Sync,
	{
		Self {
			teardown_fn: Some(Box::new(f)),
		}
	}

	pub fn new_from_box(f: Box<dyn FnOnce(&mut Context) + Send + Sync>) -> Self {
		Self {
			teardown_fn: Some(f),
		}
	}

	/// Consumes the [Teardown] without executing it, returning the stored
	/// function if it wasn't already closed.
	/// Used when the stored function is moved to somewhere else, like into a
	/// subscription.
	///
	/// It's private to ensure that it's not taken without either executing it
	/// or placing it somewhere else where execution is also guaranteed.
	#[inline]
	pub fn take(mut self) -> Option<Box<dyn FnOnce(&mut Context) + Send + Sync>> {
		self.teardown_fn.take()
	}

	/// Immediately consumes and calls the teardowns closure, leaving a None
	/// behind, rendering the teardown permamently closed.
	#[inline]
	pub fn execute(mut self, context: &mut Context) {
		if let Some(teardown) = self.teardown_fn.take() {
			(teardown)(context);
		}
	}

	#[inline]
	pub fn is_closed(&self) -> bool {
		self.teardown_fn.is_none()
	}
}

impl<Context> Default for Teardown<Context> {
	fn default() -> Self {
		Self { teardown_fn: None }
	}
}

/// Exposes and respects the original subscriptions closed-ness by storing it
/// in an option.
///
/// This means that when you convert an already closed subscription into a
/// teardown, it will be immediately dropped.
impl<S> From<S> for Teardown<S::Context>
where
	S: 'static + SubscriptionLike + Send + Sync,
{
	fn from(mut subscription: S) -> Self {
		Self {
			teardown_fn: if subscription.is_closed() {
				None
			} else {
				let closure = move |context: &mut S::Context| subscription.unsubscribe(context);
				Some(Box::new(closure))
			},
		}
	}
}
