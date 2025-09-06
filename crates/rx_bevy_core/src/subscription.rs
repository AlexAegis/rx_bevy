/// ## Why is there only a single associated context type?
///
/// Both Subscriptions and Observers in the same subscription use the same kind
/// of contexts, as signals have to be able to trigger an unsubscription. Most
/// commonly: completion and error signals should trigger an unsubscribe call.
/// And next signals sometimes trigger completion signals, so all contexts
/// must be the same.
pub trait SignalContext {
	type Context;
}

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike: SignalContext {
	fn unsubscribe(&mut self, context: &mut Self::Context);

	fn is_closed(&self) -> bool;
}

/// When implemented for a Subscriptions Context, it makes it possible to
/// unsubscribe on Drop by supplying a default context to it
pub trait DropContext {
	fn drop_context() -> Self;
}

impl DropContext for () {
	fn drop_context() -> Self {
		()
	}
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add(
		&mut self,
		subscription: impl Into<Teardown<Self::Context>>,
		context: &mut Self::Context,
	);
}

pub enum Teardown<Context> {
	Fn(Box<dyn FnOnce()>),
	Sub(Box<dyn SubscriptionLike<Context = Context>>),
}

impl<Context> Teardown<Context> {
	pub fn new<F: 'static + FnOnce()>(f: F) -> Self {
		Self::Fn(Box::new(f))
	}

	pub fn new_from_subscription(f: impl SubscriptionLike<Context = Context> + 'static) -> Self {
		Self::Sub(Box::new(f))
	}

	pub(crate) fn call(self, context: &mut Context) {
		match self {
			Self::Fn(fun) => fun(),
			Self::Sub(mut sub) => {
				sub.unsubscribe(context);
			}
		}
	}
}

impl<F, Context> From<F> for Teardown<Context>
where
	F: 'static + FnOnce(),
{
	fn from(teardown: F) -> Self {
		Self::Fn(Box::new(teardown))
	}
}

impl SignalContext for () {
	type Context = ();
}

impl SubscriptionLike for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {}
}

impl SubscriptionCollection for () {
	fn add(&mut self, subscription: impl Into<Teardown<()>>, context: &mut ()) {
		let teardown: Teardown<()> = subscription.into();
		teardown.call(context);
	}
}
