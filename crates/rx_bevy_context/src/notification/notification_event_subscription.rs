use bevy_ecs::{entity::Entity, event::Event};
use rx_core_traits::{SubscriptionNotification, Teardown, Tick};
use thiserror::Error;

use crate::BevySubscriptionContextProvider;

#[derive(Event, Clone, Debug)]
pub enum SubscriptionNotificationEvent {
	Tick(Tick),
	Unsubscribe,
	Add(Teardown<BevySubscriptionContextProvider>),
}

#[derive(Error, Debug)]
pub enum SubscriberNotificationEventError {
	#[error("Tried to send a SubscriberNotification to {0}. But it does not exist on entity {1}.")]
	NotASubscriber(String, Entity),
}

impl From<SubscriptionNotification<BevySubscriptionContextProvider>>
	for SubscriptionNotificationEvent
{
	fn from(value: SubscriptionNotification<BevySubscriptionContextProvider>) -> Self {
		match value {
			SubscriptionNotification::Tick(tick) => SubscriptionNotificationEvent::Tick(tick),
			SubscriptionNotification::Unsubscribe => SubscriptionNotificationEvent::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriptionNotificationEvent::Add(teardown),
		}
	}
}
