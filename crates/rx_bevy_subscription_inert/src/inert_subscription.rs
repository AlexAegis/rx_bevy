use std::marker::PhantomData;

use rx_bevy_core::{
	DropContext, SignalContext, SubscriptionCollection, SubscriptionLike, Teardown,
};

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
	pub fn new<S, T>(subscription: T, context: &mut Context) -> Self
	where
		S: SubscriptionLike<Context = Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
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

	fn unsubscribe(&mut self, _context: &mut Self::Context) {}

	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
	}
}

impl<Context> SubscriptionCollection for InertSubscription<Context>
where
	Context: DropContext,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		let teardown: Teardown<S, S::Context> = subscription.into();
		teardown.call(context);
	}
}
