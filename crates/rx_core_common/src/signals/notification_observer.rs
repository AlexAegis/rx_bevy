use thiserror::Error;

use crate::{Never, Observer, ObserverTerminalNotification, Signal, SubscriberNotification};

/// # [ObserverNotification]
///
///  Represents all signals a observer can observe in a materialized form
///
/// - Can be pushed into [Observer]s and [Subscribers] to trigger a call.
/// - Can try to be converted into a [ObserverTerminalNotification]
///   - Will fail for `Next`
/// - Can be converted into [SubscriberNotification]
/// - Can try to convert from a [SubscriberNotification]
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObserverNotification<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError> From<ObserverTerminalNotification<InError>> for ObserverNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: ObserverTerminalNotification<InError>) -> Self {
		match value {
			ObserverTerminalNotification::Complete => ObserverNotification::Complete,
			ObserverTerminalNotification::Error(error) => ObserverNotification::Error(error),
		}
	}
}

impl<In, InError> TryFrom<SubscriberNotification<In, InError>> for ObserverNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Error = SubscriberToObserverNotificationConversionError;

	fn try_from(
		value: SubscriberNotification<In, InError>,
	) -> Result<Self, <Self as TryFrom<SubscriberNotification<In, InError>>>::Error> {
		match value {
			SubscriberNotification::Next(next) => Ok(ObserverNotification::Next(next)),
			SubscriberNotification::Error(error) => Ok(ObserverNotification::Error(error)),
			SubscriberNotification::Complete => Ok(ObserverNotification::Complete),
			SubscriberNotification::Unsubscribe => {
				Err(SubscriberToObserverNotificationConversionError::CannotReceiveUnsubscribe)
			}
		}
	}
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriberToObserverNotificationConversionError {
	#[error("Observers are unable to receive 'Unsubscribe' notifications!")]
	CannotReceiveUnsubscribe,
}

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

#[cfg(test)]
mod test {
	use super::*;

	mod push {
		use derive_where::derive_where;
		use rx_core_macro_observer_derive::RxObserver;

		use crate::Observer;

		use super::*;

		#[derive_where(Default)]
		#[derive(RxObserver)]
		#[_rx_core_common_crate(crate)]
		#[rx_in(In)]
		#[rx_in_error(InError)]
		struct MockObserver<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			next: Option<In>,
			error: Option<InError>,
			complete: bool,
		}

		impl<In, InError> Observer for MockObserver<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			fn next(&mut self, next: Self::In) {
				self.next.replace(next);
			}

			fn error(&mut self, error: Self::InError) {
				self.error.replace(error);
			}

			fn complete(&mut self) {
				self.complete = true;
			}
		}

		#[test]
		fn should_call_next_when_pushing_a_next_notification() {
			let mut observer = MockObserver::<usize, &'static str>::default();
			observer.push(ObserverNotification::Next(1));
			assert_eq!(observer.next, Some(1));
		}

		#[test]
		fn should_call_error_when_pushing_an_error_notification() {
			let mut observer = MockObserver::<usize, &'static str>::default();
			let error = "error";
			observer.push(ObserverNotification::Error(error));
			assert_eq!(observer.error, Some(error));
		}

		#[test]
		fn should_call_complete_when_pushing_a_complete_notification() {
			let mut observer = MockObserver::<usize, &'static str>::default();
			observer.push(ObserverNotification::Complete);
			assert!(observer.complete);
		}
	}

	mod terminal_observer_notification_conversion {
		use crate::ObserverTerminalNotification;

		use super::*;

		#[test]
		fn should_convert_error() {
			let error = "error";
			let observer_notification: ObserverNotification<usize, &'static str> =
				ObserverTerminalNotification::Error(error).into();
			assert_eq!(observer_notification, ObserverNotification::Error(error));
		}

		#[test]
		fn should_convert_complete() {
			let observer_notification: ObserverNotification<usize, &'static str> =
				ObserverTerminalNotification::Complete.into();
			assert_eq!(observer_notification, ObserverNotification::Complete);
		}
	}

	mod observer_terminal_notification_conversion {
		use super::*;

		#[test]
		fn should_convert_error() {
			let error = "error";
			let observer_notification: ObserverNotification<usize, &'static str> =
				ObserverTerminalNotification::Error(error).into();
			assert_eq!(observer_notification, ObserverNotification::Error(error));
		}

		#[test]
		fn should_convert_complete() {
			let observer_notification: ObserverNotification<usize, &'static str> =
				ObserverTerminalNotification::Complete.into();
			assert_eq!(observer_notification, ObserverNotification::Complete);
		}
	}

	mod subscriber_notification_conversion {
		use super::*;

		#[test]
		fn should_convert_next() {
			let observer_notification: Result<ObserverNotification<usize>, _> =
				SubscriberNotification::Next(1).try_into();
			assert_eq!(observer_notification, Ok(ObserverNotification::Next(1)));
		}

		#[test]
		fn should_convert_error() {
			let error = "error";
			let observer_notification: Result<ObserverNotification<usize, &'static str>, _> =
				SubscriberNotification::Error(error).try_into();
			assert_eq!(
				observer_notification,
				Ok(ObserverNotification::Error(error))
			);
		}

		#[test]
		fn should_convert_complete() {
			let observer_notification: Result<ObserverNotification<usize>, _> =
				SubscriberNotification::Complete.try_into();
			assert_eq!(observer_notification, Ok(ObserverNotification::Complete));
		}

		#[test]
		fn should_not_convert_unsubscribe() {
			let observer_notification: Result<ObserverNotification<usize>, _> =
				SubscriberNotification::Unsubscribe.try_into();
			assert_eq!(
				observer_notification,
				Err(SubscriberToObserverNotificationConversionError::CannotReceiveUnsubscribe)
			);
		}
	}
}
