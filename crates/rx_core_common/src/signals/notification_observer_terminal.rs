use thiserror::Error;

use crate::{Never, Observer, ObserverNotification, Signal, SubscriberNotification};

/// # [ObserverTerminalNotification]
///
/// Represents all terminal signal events an observer can observe in a
/// materialized form.
///
/// - Can be pushed into [Observer]s and [Subscribers] to trigger a call.
/// - Can be converted into [ObserverNotification]
/// - Can be converted into [SubscriberNotification]
/// - Can try to convert from a [ObserverNotification]
/// - Can try to convert from a [SubscriberNotification]
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObserverTerminalNotification<InError = Never>
where
	InError: Signal,
{
	Error(InError),
	Complete,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObserverToObserverTerminalNotificationConversionError {
	#[error("'Next' is not a terminal signal!")]
	CannotReceiveNext,
}

impl<In, InError> TryFrom<ObserverNotification<In, InError>>
	for ObserverTerminalNotification<InError>
where
	In: Signal,
	InError: Signal,
{
	type Error = ObserverToObserverTerminalNotificationConversionError;

	fn try_from(
		value: ObserverNotification<In, InError>,
	) -> Result<Self, <ObserverTerminalNotification<InError> as TryFrom<ObserverNotification<In, InError>>>::Error>{
		match value {
			ObserverNotification::Next(_) => {
				Err(ObserverToObserverTerminalNotificationConversionError::CannotReceiveNext)
			}
			ObserverNotification::Error(error) => Ok(ObserverTerminalNotification::Error(error)),
			ObserverNotification::Complete => Ok(ObserverTerminalNotification::Complete),
		}
	}
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriberToObserverTerminalNotificationConversionError {
	#[error("'Next' is not a terminal signal!")]
	CannotReceiveNext,
	#[error("'Unsubscribe' is not a terminal signal!")]
	CannotReceiveUnsubscribe,
}

impl<In, InError> TryFrom<SubscriberNotification<In, InError>>
	for ObserverTerminalNotification<InError>
where
	In: Signal,
	InError: Signal,
{
	type Error = SubscriberToObserverTerminalNotificationConversionError;

	fn try_from(
		value: SubscriberNotification<In, InError>,
	) -> Result<Self, <ObserverTerminalNotification<InError> as TryFrom<SubscriberNotification<In, InError>>>::Error>{
		match value {
			SubscriberNotification::Next(_) => {
				Err(SubscriberToObserverTerminalNotificationConversionError::CannotReceiveNext)
			}
			SubscriberNotification::Error(error) => Ok(ObserverTerminalNotification::Error(error)),
			SubscriberNotification::Complete => Ok(ObserverTerminalNotification::Complete),
			SubscriberNotification::Unsubscribe => Err(
				SubscriberToObserverTerminalNotificationConversionError::CannotReceiveUnsubscribe,
			),
		}
	}
}

pub trait ObserverPushObserverTerminalNotificationExtention: Observer {
	fn push(&mut self, notification: impl Into<ObserverTerminalNotification<Self::InError>>);
}

impl<T> ObserverPushObserverTerminalNotificationExtention for T
where
	T: Observer,
{
	fn push(&mut self, notification: impl Into<ObserverTerminalNotification<Self::InError>>) {
		match notification.into() {
			ObserverTerminalNotification::Error(error) => self.error(error),
			ObserverTerminalNotification::Complete => self.complete(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	mod push {
		use std::marker::PhantomData;

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
			error: Option<InError>,
			complete: bool,
			_phantom_data: PhantomData<In>,
		}

		impl<In, InError> Observer for MockObserver<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			fn next(&mut self, _next: Self::In) {
				unreachable!("not used for this test")
			}

			fn error(&mut self, error: Self::InError) {
				self.error.replace(error);
			}

			fn complete(&mut self) {
				self.complete = true;
			}
		}

		#[test]
		fn should_call_error_when_pushing_an_error_notification() {
			let mut observer = MockObserver::<usize, &'static str>::default();
			let error = "error";
			observer.push(ObserverTerminalNotification::Error(error));
			assert_eq!(observer.error, Some(error));
		}

		#[test]
		fn should_call_complete_when_pushing_a_complete_notification() {
			let mut observer = MockObserver::<usize, &'static str>::default();
			observer.push(ObserverTerminalNotification::Complete);
			assert!(observer.complete);
		}
	}

	mod observer_notification_conversion {

		use super::*;

		#[test]
		fn should_convert_error() {
			let error = "error";
			let observer_terminal_notification: Result<
				ObserverTerminalNotification<&'static str>,
				_,
			> = ObserverNotification::<usize, _>::Error(error).try_into();
			assert_eq!(
				observer_terminal_notification,
				Ok(ObserverTerminalNotification::Error(error))
			);
		}

		#[test]
		fn should_convert_complete() {
			let observer_terminal_notification: Result<
				ObserverTerminalNotification<&'static str>,
				_,
			> = ObserverNotification::<usize, _>::Complete.try_into();
			assert_eq!(
				observer_terminal_notification,
				Ok(ObserverTerminalNotification::Complete)
			);
		}
	}

	mod subscriber_notification_conversion {
		use super::*;

		#[test]
		fn should_not_convert_next() {
			let observer_terminal_notification: Result<ObserverTerminalNotification, _> =
				SubscriberNotification::Next(1).try_into();
			assert_eq!(
				observer_terminal_notification,
				Err(SubscriberToObserverTerminalNotificationConversionError::CannotReceiveNext)
			);
		}

		#[test]
		fn should_convert_error() {
			let error = "error";
			let observer_terminal_notification: Result<
				ObserverTerminalNotification<&'static str>,
				_,
			> = SubscriberNotification::<usize, _>::Error(error).try_into();
			assert_eq!(
				observer_terminal_notification,
				Ok(ObserverTerminalNotification::Error(error))
			);
		}

		#[test]
		fn should_convert_complete() {
			let observer_terminal_notification: Result<ObserverTerminalNotification, _> =
				SubscriberNotification::<usize, _>::Complete.try_into();
			assert_eq!(
				observer_terminal_notification,
				Ok(ObserverTerminalNotification::Complete)
			);
		}

		#[test]
		fn should_not_convert_unsubscribe() {
			let observer_terminal_notification: Result<ObserverTerminalNotification, _> =
				SubscriberNotification::<usize, _>::Unsubscribe.try_into();
			assert_eq!(
				observer_terminal_notification,
				Err(SubscriberToObserverTerminalNotificationConversionError::CannotReceiveUnsubscribe)
			);
		}
	}
}
