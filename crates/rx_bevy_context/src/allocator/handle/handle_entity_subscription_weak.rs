use bevy_ecs::{component::Component, entity::Entity};
use rx_core_traits::{
	SubscriptionLike, SubscriptionNotification, Teardown, WithSubscriptionContext,
	allocator::handle::WeakSubscriptionHandle,
};

use crate::{
	BevySubscriptionContext, BevySubscriptionContextProvider,
	handle::unscheduled_erased_subscription_add_notification_observer_on_insert,
};

/// There's no required name component here as this handle component is expected
/// to be used on a large variety of entites.
///
/// This component intentionally does not have an on_remove hook that'd
/// unsubscribe the subscribtion, but it does have a notification observer that
/// can cause the actual subscription to be unsubscribed.
#[derive(Component)]
#[component(on_insert=unscheduled_erased_subscription_add_notification_observer_on_insert)]
pub struct WeakEntitySubscriptionHandle {
	subscription_entity: Entity,
	closed: bool,
}

impl WeakEntitySubscriptionHandle {
	pub fn new(subscription_entity: Entity) -> Self {
		Self {
			subscription_entity,
			closed: false,
		}
	}
}

impl WeakSubscriptionHandle for WeakEntitySubscriptionHandle {}

impl WithSubscriptionContext for WeakEntitySubscriptionHandle {
	type Context = BevySubscriptionContextProvider;
}

impl Clone for WeakEntitySubscriptionHandle {
	fn clone(&self) -> Self {
		Self {
			subscription_entity: self.subscription_entity.clone(),
			closed: self.closed,
		}
	}
}

impl SubscriptionLike for WeakEntitySubscriptionHandle {
	fn is_closed(&self) -> bool {
		self.closed
	}

	fn unsubscribe(&mut self, context: &mut BevySubscriptionContext<'_, '_>) {
		self.closed = true;
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Unsubscribe,
		);
	}

	fn add_teardown(
		&mut self,
		teardown: Teardown<Self::Context>,
		context: &mut BevySubscriptionContext<'_, '_>,
	) {
		context.send_subscription_notification(
			self.subscription_entity,
			SubscriptionNotification::Add(teardown),
		);
	}
}

impl Drop for WeakEntitySubscriptionHandle {
	fn drop(&mut self) {
		// Does not own its subscription so it must not do anything with it on drop.
		// It's not like it could from here anyway, but at least we
		// won't need to panic because we dropped an active subscription.

		// The component implementation of this handle must also not unsubscribe `on_remove`.
	}
}
