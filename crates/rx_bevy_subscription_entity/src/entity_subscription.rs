use std::marker::PhantomData;

use bevy_ecs::{component::Component, entity::Entity};

use rx_bevy_core::{SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable, WithContext};

use crate::ContextWithCommands;

#[derive(Component)]
pub struct EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	/// As this subscriber is stored in this entity!
	subscription_entity: Entity,
	subscription: SubscriptionData<Context>,
	phantom_data: PhantomData<&'c ()>,
}

impl<'c, Context> EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	#[inline]
	pub fn get_subscription_entity(&self) -> Entity {
		self.subscription_entity
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

impl<'c, Context> Tickable for EntitySubscription<'c, Context>
where
	Context: ContextWithCommands<'c>,
{
	fn tick(&mut self, tick: Tick, context: &mut Self::Context) {
		self.subscription.tick(tick, context);
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
