use thiserror::Error;

use crate::{Never, Observer, Signal, SubscriberNotification};

/// Represents all signal events an observer can observe in a materialized form
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObserverNotification<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError> TryFrom<SubscriberNotification<In, InError>> for ObserverNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Error = SubscriberNotificationTryFromError;

	fn try_from(
		value: SubscriberNotification<In, InError>,
	) -> Result<Self, <Self as TryFrom<SubscriberNotification<In, InError>>>::Error> {
		match value {
			SubscriberNotification::Next(next) => Ok(ObserverNotification::Next(next)),
			SubscriberNotification::Error(error) => Ok(ObserverNotification::Error(error)),
			SubscriberNotification::Complete => Ok(ObserverNotification::Complete),
			_ => Err(SubscriberNotificationTryFromError),
		}
	}
}

#[derive(Error, Debug)]
#[error(
	"SubscriberNotification variants Add, Unsubscribe and Tick can't be converted to an ObserverNotification!"
)]
pub struct SubscriberNotificationTryFromError;

pub trait ObserverPushObserverNotificationExtention: Observer {
	fn push(&mut self, notification: impl Into<ObserverNotification<Self::In, Self::InError>>);
}

impl<T> ObserverPushObserverNotificationExtention for T
where
	T: Observer,
{
	fn push(&mut self, notification: impl Into<ObserverNotification<Self::In, Self::InError>>) {
		match notification.into() {
			ObserverNotification::Next(next) => self.next(next),
			ObserverNotification::Error(error) => self.error(error),
			ObserverNotification::Complete => self.complete(),
		}
	}
}
