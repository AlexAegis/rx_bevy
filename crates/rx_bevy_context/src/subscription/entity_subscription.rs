use bevy_ecs::entity::Entity;
use bevy_log::warn;
use disqualified::ShortName;
use rx_core_macro_subscription_derive::RxSubscription;
use rx_core_traits::{
	SubscriptionClosedFlag, SubscriptionContext, SubscriptionLike, SubscriptionNotification,
	Teardown, TeardownCollection, Tick, Tickable,
};

use crate::RxBevyContext;

#[derive(RxSubscription)]
#[rx_context(RxBevyContext)]
#[rx_skip_unsubscribe_on_drop_impl]
pub struct EntitySubscription {
	closed_flag: SubscriptionClosedFlag,
	subscription_entity: Entity,
}

impl EntitySubscription {
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			closed_flag: false.into(),
			subscription_entity,
		}
	}

	pub fn into_entity(mut self) -> Entity {
		self.closed_flag.close();
		self.subscription_entity
	}
}

impl From<EntitySubscription> for Entity {
	fn from(value: EntitySubscription) -> Self {
		value.into_entity()
	}
}

impl Tickable for EntitySubscription {
	fn tick(
		&mut self,
		_tick: Tick,
		_context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		warn!(
			"Do not tick an {}, the scheduler already handles it!",
			ShortName::of::<Self>()
		);
	}
}

impl SubscriptionLike for EntitySubscription {
	#[inline]
	fn is_closed(&self) -> bool {
		*self.closed_flag
	}

	fn unsubscribe(&mut self, context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>) {
		if !self.is_closed() {
			self.closed_flag.close();
			context
				.deferred_world
				.commands()
				.entity(self.subscription_entity)
				.despawn();
		}
	}
}

impl TeardownCollection for EntitySubscription {
	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		if !self.is_closed() {
			context.send_subscription_notification(
				self.subscription_entity,
				SubscriptionNotification::Add(Some(teardown)),
			);
		} else {
			teardown.execute(context);
		}
	}
}

impl Drop for EntitySubscription {
	fn drop(&mut self) {
		// Doesn't actually own any resources, the flag is safe to close.
		self.closed_flag.close();
	}
}
