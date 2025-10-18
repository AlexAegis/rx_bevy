use bevy_ecs::{entity::Entity, event::Event};
use rx_core_traits::{
	SignalBound, SubscriberNotification, SubscriptionNotification, Teardown, Tick,
};
use thiserror::Error;

use crate::BevySubscriptionContextProvider;

/// Shorthand for [SubscriberNotificationEvent]
pub type RxSignal<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
= SubscriberNotificationEvent<In, InError>;

#[derive(Debug, Event)]
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

impl<In, InError> Into<SubscriberNotificationEvent<In, InError>>
	for SubscriberNotification<In, InError, BevySubscriptionContextProvider>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn into(self) -> SubscriberNotificationEvent<In, InError> {
		match self {
			SubscriberNotification::Next(next) => SubscriberNotificationEvent::Next(next),
			SubscriberNotification::Error(error) => SubscriberNotificationEvent::Error(error),
			SubscriberNotification::Complete => SubscriberNotificationEvent::Complete,
			SubscriberNotification::Tick(tick) => SubscriberNotificationEvent::Tick(tick),
			SubscriberNotification::Unsubscribe => SubscriberNotificationEvent::Unsubscribe,
			SubscriberNotification::Add(teardown) => SubscriberNotificationEvent::Add(teardown),
		}
	}
}

#[derive(Event)]
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
