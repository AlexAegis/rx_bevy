use bevy_derive::Deref;
use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::Event,
};
use rx_core_common::SubscriptionNotification;

// TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Clone, Deref)]
pub struct SubscriptionNotificationEvent {
	// TODO(bevy-0.17): #[event_target]
	target: Entity,
	#[deref]
	notification: SubscriptionNotification,
}

impl SubscriptionNotificationEvent {
	pub fn signal(&self) -> &SubscriptionNotification {
		&self.notification
	}
}

impl ContainsEntity for SubscriptionNotificationEvent {
	fn entity(&self) -> Entity {
		self.target
	}
}

impl SubscriptionNotificationEvent {
	#[inline]
	pub fn from_notification(notification: SubscriptionNotification, target: Entity) -> Self {
		Self {
			notification,
			target,
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
		let entity = Entity::from_raw(5);
		let notification = SubscriptionNotification::Unsubscribe;
		let event = SubscriptionNotificationEvent::from_notification(notification, entity);
		assert_eq!(event.entity(), entity);
		assert_eq!(*event, notification);
	}

	#[test]
	fn it_should_be_able_to_convert_from_a_subscription_notification() {
		let entity = Entity::from_raw(10);
		let event = SubscriptionNotificationEvent::from_notification(
			SubscriptionNotification::Unsubscribe,
			entity,
		);
		let notification: SubscriptionNotification = event.into();
		assert_eq!(notification, SubscriptionNotification::Unsubscribe);
	}
}
