use bevy_ecs::event::Event;
use rx_bevy_core::{
	ObserverNotification, SignalBound, SubscriberNotification, SubscriptionNotification, Teardown,
	Tick,
};

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

#[derive(Event)]
pub enum ObserverNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
}

impl<In, InError> Into<ObserverNotificationEvent<In, InError>> for ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn into(self) -> ObserverNotificationEvent<In, InError> {
		match self {
			ObserverNotification::Next(next) => ObserverNotificationEvent::Next(next),
			ObserverNotification::Error(error) => ObserverNotificationEvent::Error(error),
			ObserverNotification::Complete => ObserverNotificationEvent::Complete,
			ObserverNotification::Tick(tick) => ObserverNotificationEvent::Tick(tick),
		}
	}
}
