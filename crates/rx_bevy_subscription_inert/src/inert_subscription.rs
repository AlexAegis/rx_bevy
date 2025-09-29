use std::marker::PhantomData;

use rx_bevy_core::{DropContext, SignalContext, SubscriptionLike, Teardown};

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
	Context: DropContext,
{
	// TODO: Check every PhantomData for variance
	_phantom_data: PhantomData<*mut Context>,
}

impl<Context> InertSubscription<Context>
where
	Context: DropContext,
{
	pub fn new<T>(subscription: T, context: &mut Context) -> Self
	where
		T: Into<Teardown<Context>>,
	{
		let teardown: Teardown<Context> = subscription.into();
		teardown.call(context);

		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> Default for InertSubscription<Context>
where
	Context: DropContext,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<Context> SignalContext for InertSubscription<Context>
where
	Context: DropContext,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for InertSubscription<Context>
where
	Context: DropContext,
{
	fn is_closed(&self) -> bool {
		true
	}

	fn unsubscribe(&mut self, _context: &mut Self::Context) {
		// Does not need to do anything on unsubscribe
	}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}

	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		// The added teardown is executed immediately as this subscription is always closed.
		teardown.call(context);
	}
}
