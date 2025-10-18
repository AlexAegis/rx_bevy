use bevy_ecs::{entity::Entity, event::Event};
use rx_core_traits::{
	SignalBound, SubscriberNotification, SubscriptionNotification, Teardown, Tick,
};
use thiserror::Error;

use crate::{BevySubscriptionContextProvider, context::EntitySubscriptionContextAccessProvider};

#[derive(Event)]
pub enum SubscriberNotificationEvent<In, InError, ContextAccess>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Option<Teardown<BevySubscriptionContextProvider<ContextAccess>>>),
}

impl<In, InError, ContextAccess> Into<SubscriberNotificationEvent<In, InError, ContextAccess>>
	for SubscriberNotification<In, InError, BevySubscriptionContextProvider<ContextAccess>>
where
	In: SignalBound,
	InError: SignalBound,
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn into(self) -> SubscriberNotificationEvent<In, InError, ContextAccess> {
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

/// TODO: This is currently unused
#[derive(Event)]
pub enum SubscriptionNotificationEvent<ContextAccess>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	Tick(Tick),
	Unsubscribe,
	Add(Teardown<BevySubscriptionContextProvider<ContextAccess>>),
}

impl<ContextAccess> Into<SubscriptionNotificationEvent<ContextAccess>>
	for SubscriptionNotification<BevySubscriptionContextProvider<ContextAccess>>
where
	ContextAccess: 'static + EntitySubscriptionContextAccessProvider,
{
	fn into(self) -> SubscriptionNotificationEvent<ContextAccess> {
		match self {
			SubscriptionNotification::Tick(tick) => SubscriptionNotificationEvent::Tick(tick),
			SubscriptionNotification::Unsubscribe => SubscriptionNotificationEvent::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriptionNotificationEvent::Add(teardown),
		}
	}
}

/// A simplified subscription notification that doesn't allow allow adding new
/// teardowns, but also doesn't need to know about the context used.
#[derive(Event)]
pub enum ObservableSubscriptionNotificationEvent {
	Tick(Tick),
	Unsubscribe,
}

#[derive(Error, Debug)]
pub enum SubscriptionNotificationEventError {
	#[error(
		"Tried to send a SubscriptionNotification to {0}. But it does not exist on entity {1}."
	)]
	NotASubscription(String, Entity),
}
