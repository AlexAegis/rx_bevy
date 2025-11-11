use thiserror::Error;

use crate::{Never, Observer, SignalBound, SubscriberNotification, context::SubscriptionContext};

/// Represents all signal events an observer can observe in a materialized form
#[derive(Debug, Clone)]
pub enum ObserverNotification<In, InError = Never>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError, Context> TryFrom<SubscriberNotification<In, InError, Context>>
	for ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
	Context: SubscriptionContext,
{
	type Error = SubscriberNotificationTryFromError;

	fn try_from(
		value: SubscriberNotification<In, InError, Context>,
	) -> Result<Self, <Self as TryFrom<SubscriberNotification<In, InError, Context>>>::Error> {
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
	fn push(
		&mut self,
		notification: impl Into<ObserverNotification<Self::In, Self::InError>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	);
}

impl<T> ObserverPushObserverNotificationExtention for T
where
	T: Observer,
{
	fn push(
		&mut self,
		notification: impl Into<ObserverNotification<Self::In, Self::InError>>,
		context: &mut <Self::Context as SubscriptionContext>::Item<'_, '_>,
	) {
		match notification.into() {
			ObserverNotification::Next(next) => self.next(next, context),
			ObserverNotification::Error(error) => self.error(error, context),
			ObserverNotification::Complete => self.complete(context),
		}
	}
}
