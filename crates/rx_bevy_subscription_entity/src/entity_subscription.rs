use std::marker::PhantomData;

use bevy_ecs::component::Component;

use rx_bevy_core::{Subscription, SubscriptionLike, Teardown, WithContext};

use rx_bevy_context_command::ContextWithCommands;

#[derive(Component)]
pub struct EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	subscription: Subscription<Context>,
	phantom_data: PhantomData<&'c Context>,
}

impl<'c, Context> Default for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn default() -> Self {
		Self {
			subscription: Subscription::<Context>::default(),
			phantom_data: PhantomData,
		}
	}
}

impl<'c, Context> WithContext for EntitySubscription<'c, Context>
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
	fn get_context_to_unsubscribe_on_drop(&mut self) -> Self::Context {
		Context::create_context_to_unsubscribe_on_drop()
	}
}

impl<'c, Context> Drop for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn drop(&mut self) {
		// Only panics when the `dev_panic_on_dropped_active_subscriptions`
		// feature is active, otherwise it just prints a warning.

		if !self.is_closed() {
			let message = format!(
				"{} was dropped without unsubscribing first!",
				short_type_name::short_type_name::<Self>()
			);
			#[cfg(not(feature = "dev_panic_on_dropped_active_subscriptions"))]
			bevy_log::warn!("{}", message);

			#[cfg(feature = "dev_panic_on_dropped_active_subscriptions")]
			panic!("{}", message);
		}
	}
}
