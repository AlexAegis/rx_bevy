use bevy_derive::Deref;
use bevy_ecs::{entity::Entity, event::EntityEvent};
use rx_core_common::SubscriptionNotification;

#[derive(EntityEvent, Clone, Deref)]
pub struct SubscriptionNotificationEvent {
	entity: Entity,
	#[deref]
	notification: SubscriptionNotification,
}

impl SubscriptionNotificationEvent {
	pub fn entity(&self) -> Entity {
		self.entity
	}

	pub fn signal(&self) -> &SubscriptionNotification {
		&self.notification
	}
}

impl SubscriptionNotificationEvent {
	#[inline]
	pub fn from_notification(notification: SubscriptionNotification, target: Entity) -> Self {
		Self {
			notification,
			entity: target,
		}
	}
}

impl From<SubscriptionNotificationEvent> for SubscriptionNotification {
	fn from(event: SubscriptionNotificationEvent) -> Self {
		event.notification
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn it_should_create_a_subscription_notification_event() {
		let entity = Entity::from_raw_u32(5).unwrap();
		let notification = SubscriptionNotification::Unsubscribe;
		let event = SubscriptionNotificationEvent::from_notification(notification, entity);
		assert_eq!(event.entity(), entity);
		assert_eq!(*event, notification);
	}

	#[test]
	fn it_should_be_able_to_convert_from_a_subscription_notification() {
		let entity = Entity::from_raw_u32(10).unwrap();
		let event = SubscriptionNotificationEvent::from_notification(
			SubscriptionNotification::Unsubscribe,
			entity,
		);
		let notification: SubscriptionNotification = event.into();
		assert_eq!(notification, SubscriptionNotification::Unsubscribe);
	}
}
