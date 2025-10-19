use bevy_ecs::event::Event;
use rx_core_traits::SubscriptionNotification;

use crate::{BevySubscriptionContextProvider, SubscriptionNotificationEvent};

#[derive(Event, Clone, Debug)]
pub struct ConsumableSubscriptionNotificationEvent {
	notification: Option<SubscriptionNotificationEvent>,
}

impl ConsumableSubscriptionNotificationEvent {
	pub fn take(&mut self) -> Option<SubscriptionNotificationEvent> {
		self.notification.take()
	}
}

impl From<SubscriptionNotification<BevySubscriptionContextProvider>>
	for ConsumableSubscriptionNotificationEvent
{
	fn from(value: SubscriptionNotification<BevySubscriptionContextProvider>) -> Self {
		let notification_event: SubscriptionNotificationEvent = value.into();

		ConsumableSubscriptionNotificationEvent {
			notification: Some(notification_event),
		}
	}
}

impl From<SubscriptionNotificationEvent> for ConsumableSubscriptionNotificationEvent {
	fn from(value: SubscriptionNotificationEvent) -> Self {
		ConsumableSubscriptionNotificationEvent {
			notification: Some(value),
		}
	}
}
