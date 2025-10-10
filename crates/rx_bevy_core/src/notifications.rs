use crate::{Teardown, Tick};

/// Represents a signal event in a materialized form.
/// Useful for
#[derive(Debug)]
pub enum ObserverNotification<In, InError> {
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
}

/// Represents a signal event in a materialized form
#[derive(Debug)]
pub enum SubscriptionNotification<Context> {
	Unsubscribe,
	Add(Teardown<Context>),
	Tick(Tick),
}

/// Represents all signal events a subscriber can observe in a materialized form
#[derive(Debug)]
pub enum SubscriberNotification<In, InError, Context> {
	Next(In),
	Error(InError),
	Complete,
	Tick(Tick),
	Unsubscribe,
	Add(Option<Teardown<Context>>),
}

impl<In, InError, Context> From<ObserverNotification<In, InError>>
	for SubscriberNotification<In, InError, Context>
{
	fn from(value: ObserverNotification<In, InError>) -> Self {
		match value {
			ObserverNotification::Next(next) => SubscriberNotification::Next(next),
			ObserverNotification::Error(error) => SubscriberNotification::Error(error),
			ObserverNotification::Complete => SubscriberNotification::Complete,
			ObserverNotification::Tick(tick) => SubscriberNotification::Tick(tick),
		}
	}
}

impl<In, InError, Context> From<SubscriptionNotification<Context>>
	for SubscriberNotification<In, InError, Context>
{
	fn from(value: SubscriptionNotification<Context>) -> Self {
		match value {
			SubscriptionNotification::Unsubscribe => SubscriberNotification::Unsubscribe,
			SubscriptionNotification::Add(teardown) => SubscriberNotification::Add(Some(teardown)),
			SubscriptionNotification::Tick(tick) => SubscriberNotification::Tick(tick),
		}
	}
}
