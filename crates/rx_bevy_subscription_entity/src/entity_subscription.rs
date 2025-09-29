use std::marker::PhantomData;

use bevy_ecs::component::Component;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, InnerSubscription, SignalContext, SubscriptionLike, Teardown,
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
	#[inline]
	fn is_closed(&self) -> bool {
		self.subscription.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut Context) {
		self.subscription.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(&mut self, teardown: Teardown<Self::Context>, context: &mut Self::Context) {
		self.subscription.add_teardown(teardown, context);
	}

	#[inline]
	fn get_unsubscribe_context(&mut self) -> Self::Context {
		Context::get_context_for_drop()
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
