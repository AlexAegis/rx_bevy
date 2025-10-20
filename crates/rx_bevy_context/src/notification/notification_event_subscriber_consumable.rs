use bevy_ecs::event::Event;
use rx_core_traits::{SignalBound, SubscriberNotification};

use crate::{BevySubscriptionContextProvider, SubscriberNotificationEvent};

#[derive(Event, Clone, Debug)]
pub struct ConsumableSubscriberNotificationEvent<In, InError = ()>
where
	In: SignalBound,
	InError: SignalBound,
{
	notification: Option<SubscriberNotificationEvent<In, InError>>,
}

impl<In, InError> ConsumableSubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	pub fn consume(&mut self) -> SubscriberNotificationEvent<In, InError> {
		self.notification
			.take()
			.expect("Notification was already consumed!")
	}
}

impl<In, InError> From<SubscriberNotification<In, InError, BevySubscriptionContextProvider>>
	for ConsumableSubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: SubscriberNotification<In, InError, BevySubscriptionContextProvider>) -> Self {
		let notification_event: SubscriberNotificationEvent<In, InError> = value.into();

		ConsumableSubscriberNotificationEvent {
			notification: Some(notification_event),
		}
	}
}
