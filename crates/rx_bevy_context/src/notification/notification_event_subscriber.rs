use bevy_ecs::{entity::Entity, event::Event};
use rx_core_traits::{SignalBound, SubscriberNotification, Teardown, Tick};
use thiserror::Error;

use crate::BevySubscriptionContextProvider;

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

#[derive(Error, Debug)]
pub enum SubscriptionNotificationEventError {
	#[error(
		"Tried to send a SubscriptionNotification to {0}. But it does not exist on entity {1}."
	)]
	NotASubscription(String, Entity),
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

impl<In, InError> From<SubscriberNotificationEvent<In, InError>>
	for SubscriberNotification<In, InError, BevySubscriptionContextProvider>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: SubscriberNotificationEvent<In, InError>) -> Self {
		match value {
			SubscriberNotificationEvent::Next(next) => SubscriberNotification::Next(next),
			SubscriberNotificationEvent::Error(error) => SubscriberNotification::Error(error),
			SubscriberNotificationEvent::Complete => SubscriberNotification::Complete,
			SubscriberNotificationEvent::Tick(tick) => SubscriberNotification::Tick(tick),
			SubscriberNotificationEvent::Unsubscribe => SubscriberNotification::Unsubscribe,
			SubscriberNotificationEvent::Add(teardown) => SubscriberNotification::Add(teardown),
		}
	}
}
