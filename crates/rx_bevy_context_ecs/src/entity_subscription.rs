use bevy_ecs::{component::Component, entity::Entity};

use rx_bevy_core::{
	SubscriptionData, SubscriptionLike, Teardown, Tick, Tickable, context::WithSubscriptionContext,
	prelude::SubscriptionContext,
};

use crate::BevySubscriptionContextProvider;

#[derive(Component)]
pub struct EntitySubscription {
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	/// As this subscriber is stored in this entity!
	self_entity: Entity,
	subscription: SubscriptionData<BevySubscriptionContextProvider>,
}

impl EntitySubscription {
	pub fn new(self_entity: Entity) -> Self {
		Self {
			self_entity,

			subscription: SubscriptionData::default(),
		}
	}

	pub fn new_with_teardown(
		self_entity: Entity,
		teardown: Teardown<BevySubscriptionContextProvider>,
	) -> Self {
		Self {
			self_entity,
			subscription: SubscriptionData::new_with_teardown(teardown),
		}
	}

	#[inline]
	pub fn get_self_entity(&self) -> Entity {
		self.self_entity
	}
}

impl WithSubscriptionContext for EntitySubscription {
	type Context = BevySubscriptionContextProvider;
}

impl SubscriptionLike for EntitySubscription {
	#[inline]
	fn is_closed(&self) -> bool {
		true
		//self.subscription.is_closed()
	}

	#[inline]
	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		//self.subscription.unsubscribe(context);
	}

	#[inline]
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_>,
	) {
		//self.subscription.add_teardown(teardown, context);
	}
}

impl Tickable for EntitySubscription {
	fn tick(&mut self, tick: Tick, context: &mut <Self::Context as SubscriptionContext>::Item<'_>) {
		//self.subscription.tick(tick, context);
	}
}

impl Drop for EntitySubscription {
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
