use crate::{ObserverNotification, Signal, Subscriber, SubscriptionNotification};

/// Represents all signal events a subscriber can observe in a materialized form
#[derive(Debug, PartialEq)]
pub enum SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	Next(In),
	Error(InError),
	Complete,
	Unsubscribe,
}

impl<In, InError> Clone for SubscriberNotification<In, InError>
where
	In: Signal + Clone,
	InError: Signal + Clone,
{
	fn clone(&self) -> Self {
		match self {
			Self::Next(next) => Self::Next(next.clone()),
			Self::Error(error) => Self::Error(error.clone()),
			Self::Complete => Self::Complete,
			Self::Unsubscribe => Self::Unsubscribe,
		}
	}
}

impl<In, InError> From<ObserverNotification<In, InError>> for SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: ObserverNotification<In, InError>) -> Self {
		match value {
			ObserverNotification::Next(next) => SubscriberNotification::Next(next),
			ObserverNotification::Error(error) => SubscriberNotification::Error(error),
			ObserverNotification::Complete => SubscriberNotification::Complete,
		}
	}
}

impl<In, InError> From<SubscriptionNotification> for SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: SubscriptionNotification) -> Self {
		match value {
			SubscriptionNotification::Unsubscribe => SubscriberNotification::Unsubscribe,
		}
	}
}

pub trait SubscriberPushNotificationExtention: Subscriber {
	fn push(&mut self, notification: impl Into<SubscriberNotification<Self::In, Self::InError>>);
}

impl<T> SubscriberPushNotificationExtention for T
where
	T: Subscriber,
{
	fn push(&mut self, notification: impl Into<SubscriberNotification<Self::In, Self::InError>>) {
		match notification.into() {
			SubscriberNotification::Next(next) => self.next(next),
			SubscriberNotification::Error(error) => self.error(error),
			SubscriberNotification::Complete => self.complete(),
			SubscriberNotification::Unsubscribe => self.unsubscribe(),
		}
	}
}
