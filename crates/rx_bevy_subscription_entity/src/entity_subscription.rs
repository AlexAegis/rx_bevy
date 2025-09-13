use bevy_ecs::component::Component;

use rx_bevy_core::{
	AssertSubscriptionClosedOnDrop, InnerSubscription, SignalContext, SubscriptionCollection,
	SubscriptionLike,
};

use rx_bevy_context_command::ContextWithCommands;

#[derive(Component)]
pub struct EntitySubscription<'c, Context>(InnerSubscription<'c, Context>)
where
	Context: ContextWithCommands<'c>;

impl<'c, Context> Default for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn default() -> Self {
		Self(InnerSubscription::<Context>::default())
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
		self.0.is_closed()
	}

	fn unsubscribe(&mut self, context: &mut Context) {
		self.0.unsubscribe(context);
	}
}

impl<'c, Context> SubscriptionCollection<'c> for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn add<S: 'c + SubscriptionLike<Context = Self::Context>>(
		&mut self,
		subscription: S,
		context: &mut Self::Context,
	) {
		self.0.add(subscription, context);
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
