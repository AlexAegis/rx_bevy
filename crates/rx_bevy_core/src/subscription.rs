/// ## Why is there only a single associated context type?
///
/// Both Subscriptions and Observers in the same subscription use the same kind
/// of contexts, as signals have to be able to trigger an unsubscription. Most
/// commonly: completion and error signals should trigger an unsubscribe call.
/// And next signals sometimes trigger completion signals, so all contexts
/// must be the same.
pub trait SignalContext {
	type Context<'c>;
}

/// A [SubscriptionLike] is something that can be "unsubscribed" from, which will
/// close it, rendering it no longer operational, and safe to drop
/// but it doesn't actually execute any teardown logic beyond its own, it is
/// primarily used by operators.
pub trait SubscriptionLike: SignalContext {
	fn unsubscribe<'c>(&mut self, context: &mut Self::Context<'c>);

	fn is_closed(&self) -> bool;
}

/// For subscriptions where the [Subscription] itself contains everything
/// needed for it to unsubscribe, this trait can enable unsubscribe-on-drop
/// behavior.
pub trait DropContextFromSubscription: SignalContext {
	fn get_unsubscribe_context<'c>(&mut self) -> Self::Context<'c>;
}

impl DropContextFromSubscription for () {
	fn get_unsubscribe_context<'c>(&mut self) -> Self::Context<'c> {
		()
	}
}

/// In addition to [ContextFromSubscription], this trait denotes contexts for
/// for dropped [Subscription]s. For example when the context is just `()`.
pub trait DropContext: 'static {
	fn get_context_for_drop() -> Self;
}

impl DropContext for () {
	fn get_context_for_drop() -> Self {
		()
	}
}

pub trait SubscriptionCollection: SubscriptionLike {
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	);
}

pub enum Teardown<Context> {
	Fn(Box<dyn FnOnce(&mut Context)>),
}

impl<Context> Teardown<Context> {
	pub fn new<F: 'static + FnOnce(&mut Context)>(f: F) -> Self {
		Self::Fn(Box::new(f))
	}

	pub(crate) fn call(self, context: &mut Context) {
		match self {
			Self::Fn(fun) => fun(context),
		}
	}
}

impl<F, Context> From<F> for Teardown<Context>
where
	F: 'static + FnOnce(&mut Context),
{
	fn from(teardown: F) -> Self {
		Self::Fn(Box::new(teardown))
	}
}

impl SignalContext for () {
	type Context<'c> = ();
}

impl SubscriptionLike for () {
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe<'c>(&mut self, _context: &mut Self::Context<'c>) {}
}

impl SubscriptionCollection for () {
	fn add<'c>(
		&mut self,
		subscription: impl Into<Teardown<Self::Context<'c>>>,
		context: &mut Self::Context<'c>,
	) {
		let teardown: Teardown<()> = subscription.into();
		teardown.call(context);
	}
}
