use bevy_ecs::{entity::Entity, event::Event};
use rx_core_traits::{
	SignalBound, SubscriberNotification, SubscriptionNotification, Teardown, Tick,
};
use thiserror::Error;

use crate::BevySubscriptionContextProvider;

#[derive(Event, Clone, Debug)]
pub enum RxSignal<In, InError = ()>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError> From<RxSignal<In, InError>> for SubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		match value {
			RxSignal::Next(next) => SubscriberNotificationEvent::Next(next),
			RxSignal::Error(error) => SubscriberNotificationEvent::Error(error),
			RxSignal::Complete => SubscriberNotificationEvent::Complete,
		}
	}
}

/// Shorthand for [SubscriberNotificationEvent]
// TODO: use this or don't pub type RxSignal<In, InError> = InternalSubscriberNotificationEvent<In, InError>;

#[derive(Event, Clone, Debug)]
pub(crate) struct ConsumableSubscriberNotificationEvent<In, InError = ()>
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
	pub fn take(&mut self) -> Option<SubscriberNotificationEvent<In, InError>> {
		self.notification.take()
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

/// Since events are passed around as references and signals must be owned, we
/// can levarage the fact that these events are sent only once, and only to
/// one destination and let the `In` and `InError` signals be taken out of the
/// event.
#[derive(Event, Clone, Debug)]
pub enum SubscriberNotificationEvent<In, InError = ()>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Option<Teardown<BevySubscriptionContextProvider>>),
}

impl<In, InError> From<SubscriberNotification<In, InError, BevySubscriptionContextProvider>>
	for SubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: SubscriberNotification<In, InError, BevySubscriptionContextProvider>) -> Self {
		match value {
			SubscriberNotification::Next(next) => SubscriberNotificationEvent::Next(next),
			SubscriberNotification::Error(error) => SubscriberNotificationEvent::Error(error),
			SubscriberNotification::Complete => SubscriberNotificationEvent::Complete,
			SubscriberNotification::Tick(tick) => SubscriberNotificationEvent::Tick(tick),
			SubscriberNotification::Unsubscribe => SubscriberNotificationEvent::Unsubscribe,
			SubscriberNotification::Add(teardown) => SubscriberNotificationEvent::Add(teardown),
		}
	}
}

#[derive(Event, Clone, Debug)]
pub(crate) struct ConsumableSubscriptionNotificationEvent {
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

#[derive(Event, Clone, Debug)]
pub enum SubscriptionNotificationEvent {
	Tick(Tick),
	Unsubscribe,
	Add(Teardown<BevySubscriptionContextProvider>),
}

impl Into<SubscriptionNotificationEvent>
	for SubscriptionNotification<BevySubscriptionContextProvider>
{
	fn into(self) -> SubscriptionNotificationEvent {
		match self {
			SubscriptionNotification::Tick(tick) => SubscriptionNotificationEvent::Tick(tick),
			SubscriptionNotification::Unsubscribe => SubscriptionNotificationEvent::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriptionNotificationEvent::Add(teardown),
		}
	}
}

#[derive(Error, Debug)]
pub enum SubscriptionNotificationEventError {
	#[error(
		"Tried to send a SubscriptionNotification to {0}. But it does not exist on entity {1}."
	)]
	NotASubscription(String, Entity),
}

#[derive(Error, Debug)]
pub enum SubscriberNotificationEventError {
	#[error("Tried to send a SubscriberNotification to {0}. But it does not exist on entity {1}.")]
	NotASubscriber(String, Entity),
}
