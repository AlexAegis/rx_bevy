use bevy_ecs::component::Component;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, InnerSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike,
};

use rx_bevy_context_command::ContextWithCommands;

#[derive(Component)]
pub struct EntitySubscription<Context>(InnerSubscription<Context>)
where
	Context: for<'c> ContextWithCommands<'c>;

impl<Context> Default for EntitySubscription<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	fn default() -> Self {
		Self(InnerSubscription::<Context>::default())
	}
}

impl<Context> SignalContext for EntitySubscription<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	type Context = Context;
}

impl<Context> SubscriptionLike for EntitySubscription<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	fn is_closed(&self) -> bool {
		self.0.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		self.0.unsubscribe(context);
	}
}

impl<Context> SubscriptionCollection for EntitySubscription<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	fn add<S: 'static + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: impl Into<S>,
		context: &mut Self::Context,
	) {
		self.0.add(subscription, context);
	}
}

impl<Context> Drop for EntitySubscription<Context>
where
	Context: for<'c> ContextWithCommands<'c>,
{
	fn drop(&mut self) {
		// Only panics when the `dev_panic_on_dropped_active_subscriptions`
		// feature is active, otherwise it just prints a warning.
		self.assert_closed_when_dropped();
	}
}
