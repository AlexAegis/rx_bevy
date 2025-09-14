use std::marker::PhantomData;

use bevy_ecs::component::Component;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, InnerSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike, Teardown,
};

use rx_bevy_context_command::ContextWithCommands;

#[derive(Component)]
pub struct EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	subscription: InnerSubscription<Context>,
	phantom_data: PhantomData<&'c Context>,
}

impl<'c, Context> Default for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn default() -> Self {
		Self {
			subscription: InnerSubscription::<Context>::default(),
			phantom_data: PhantomData,
		}
	}
}

impl<'c, Context> SignalContext for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	type Context = Context;
}

impl<'c, Context> SubscriptionLike for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		self.subscription.unsubscribe(context);
	}
}

impl<'c, Context> SubscriptionCollection for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn add<S, T>(&mut self, subscription: T, context: &mut Self::Context)
	where
		S: SubscriptionLike<Context = Self::Context>,
		T: Into<Teardown<S, S::Context>>,
	{
		self.subscription.add(subscription, context);
	}
}

impl<'c, Context> Drop for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn drop(&mut self) {
		// Only panics when the `dev_panic_on_dropped_active_subscriptions`
		// feature is active, otherwise it just prints a warning.
		self.assert_closed_when_dropped();
	}
}
