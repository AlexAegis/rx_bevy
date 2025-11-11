use bevy_ecs::event::Event;
use rx_core_traits::SubscriptionNotification;

use crate::BevySubscriptionContextProvider;

#[derive(Event, Clone)]
pub struct SubscriptionNotificationEvent {
	pub(crate) notification: SubscriptionNotification<BevySubscriptionContextProvider>,
}

impl From<SubscriptionNotification<BevySubscriptionContextProvider>>
	for SubscriptionNotificationEvent
{
	fn from(notification: SubscriptionNotification<BevySubscriptionContextProvider>) -> Self {
		Self { notification }
	}
}

impl From<SubscriptionNotificationEvent>
	for SubscriptionNotification<BevySubscriptionContextProvider>
{
	fn from(event: SubscriptionNotificationEvent) -> Self {
		event.notification
	}
}
