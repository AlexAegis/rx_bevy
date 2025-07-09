use bevy_ecs::{entity::Entity, system::Commands};
use derive_where::derive_where;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg_attr(feature = "debug", derive_where(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionContext<'a, 'w, 's> {
	#[derive_where(skip)]
	pub commands: &'a mut Commands<'w, 's>,
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,

	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

// TODO: So that you can just .next stuff instead of emitting values by hand
//impl Observer for SubscriptionContext {}

#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct SubscriptionEntityContext {
	/// "This" entity
	pub observable_entity: Entity,
	/// "Destination" entity
	pub subscriber_entity: Entity,
	/// Despawning this stops the subscription, and is equivalent of an Unsubscribe
	pub subscription_entity: Entity,
}

impl SubscriptionEntityContext {
	pub fn upgrade<'a, 'w, 's>(
		self,
		commands: &'a mut Commands<'w, 's>,
	) -> SubscriptionContext<'a, 'w, 's> {
		SubscriptionContext {
			commands,
			observable_entity: self.observable_entity,
			subscriber_entity: self.subscriber_entity,
			subscription_entity: self.subscription_entity,
		}
	}
}
