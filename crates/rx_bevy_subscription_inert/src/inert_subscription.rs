use std::marker::PhantomData;

use rx_bevy_core::{SignalContext, SubscriptionLike, Teardown, WithContext};

/// A [InertSubscription] is a permanently closed [Subscription] that immediately
/// runs any [Teardown] you may add into it.
/// It is used for [Observable]s that emit all their values, complete and
/// unsubscribe immediately on subscribe.
/// This aspect lets us safely ignore the drop-safety of the context used, as
/// subscriptions made with drop-unsafe contexts can (obviously) be dropped once
/// they are unsubscribed, and that is guaranteed here.
#[derive(Clone)]
pub struct InertSubscription<Context>
where
	Context: SignalContext,
{
	// TODO: Check every PhantomData for variance
	_phantom_data: PhantomData<*mut Context>,
}

impl<Context> InertSubscription<Context>
where
	Context: SignalContext,
{
	pub fn new<T>(subscription: T, context: &mut Context) -> Self
	where
		T: Into<Teardown<Context>>,
	{
		let teardown: Teardown<Context> = subscription.into();
		teardown.execute(context);

		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> Default for InertSubscription<Context>
where
	Context: SignalContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> WithContext for InertSubscription<Context>
where
	Context: SignalContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InertSubscription<Context>
where
	Context: SignalContext,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		// Does not need to do anything on unsubscribe
	}

	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		// The added teardown is executed immediately as this subscription is always closed.
		teardown.execute(context);
	}
}
