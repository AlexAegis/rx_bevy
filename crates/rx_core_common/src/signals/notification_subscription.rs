use thiserror::Error;

use crate::{Signal, SubscriberNotification, SubscriptionLike};

/// # [SubscriptionNotification]
///
/// Represents the materialized form of an
/// [`unsubscribe`][crate::SubscriptionLike::unsubscribe] call.
///
/// - Can be pushed into [SubscriptionLike]s to trigger a call.
/// - Can be converted into [SubscriberNotification]
/// - Can try to convert from [SubscriberNotification]
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriptionNotification {
	Unsubscribe,
}

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriberToSubscriptionNotificationConversionError {
	#[error("Subscriptions are unable to receive 'Next' notifications!")]
	CannotReceiveNext,
	#[error("Subscriptions are unable to receive 'Error' notifications!")]
	CannotReceiveError,
	#[error("Subscriptions are unable to receive 'Complete' notifications!")]
	CannotReceiveComplete,
}

impl<In, InError> TryFrom<SubscriberNotification<In, InError>> for SubscriptionNotification
where
	In: Signal,
	InError: Signal,
{
	type Error = SubscriberToSubscriptionNotificationConversionError;

	fn try_from(value: SubscriberNotification<In, InError>) -> Result<Self, Self::Error> {
		match value {
			SubscriberNotification::Unsubscribe => Ok(SubscriptionNotification::Unsubscribe),
			SubscriberNotification::Next(_) => {
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveNext)
			}
			SubscriberNotification::Error(_) => {
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveError)
			}
			SubscriberNotification::Complete => {
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveComplete)
			}
		}
	}
}

pub trait SubscriptionLikePushNotificationExtention: SubscriptionLike {
	fn push(&mut self, notification: impl Into<SubscriptionNotification>);
}

impl<T> SubscriptionLikePushNotificationExtention for T
where
	T: SubscriptionLike,
{
	fn push(&mut self, notification: impl Into<SubscriptionNotification>) {
		match notification.into() {
			SubscriptionNotification::Unsubscribe => self.unsubscribe(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	mod push {
		use rx_core_macro_subscription_derive::RxSubscription;

		use crate::SubscriptionLike;

		use super::*;

		#[derive(RxSubscription, Default)]
		#[_rx_core_common_crate(crate)]
		struct MockSubscription {
			unsubscribed: bool,
		}

		impl SubscriptionLike for MockSubscription {
			fn is_closed(&self) -> bool {
				self.unsubscribed
			}

			fn unsubscribe(&mut self) {
				self.unsubscribed = true;
			}
		}

		#[test]
		fn should_call_unsubscribe_when_pushing_an_unsubscribe_notification() {
			let mut subscription = MockSubscription::default();
			subscription.push(SubscriptionNotification::Unsubscribe);
			assert!(subscription.is_closed());
		}
	}

	mod subscriber_notification_try_conversion {
		use super::*;

		#[test]
		fn should_not_convert_next() {
			let subscription_notification: Result<SubscriptionNotification, _> =
				SubscriberNotification::<usize>::Next(1).try_into();
			assert_eq!(
				subscription_notification,
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveNext)
			);
		}

		#[test]
		fn should_not_convert_error() {
			let subscription_notification: Result<SubscriptionNotification, _> =
				SubscriberNotification::<usize, &'static str>::Error("error").try_into();
			assert_eq!(
				subscription_notification,
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveError)
			);
		}

		#[test]
		fn should_not_convert_complete() {
			let subscription_notification: Result<SubscriptionNotification, _> =
				SubscriberNotification::<usize, &'static str>::Complete.try_into();
			assert_eq!(
				subscription_notification,
				Err(SubscriberToSubscriptionNotificationConversionError::CannotReceiveComplete)
			);
		}

		#[test]
		fn should_convert_unsubscribe_successfully() {
			let subscription_notification: Result<SubscriptionNotification, _> =
				SubscriberNotification::<usize>::Unsubscribe.try_into();
			assert_eq!(
				subscription_notification,
				Ok(SubscriptionNotification::Unsubscribe)
			);
		}
	}
}
