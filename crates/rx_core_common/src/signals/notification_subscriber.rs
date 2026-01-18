use crate::{
	Never, ObserverNotification, ObserverTerminalNotification, Signal, Subscriber,
	SubscriptionNotification,
};

/// # [SubscriberNotification]
///
///  Represents all signals a subscriber can observe in a materialized form
///
/// - Can be pushed into [Subscriber]s to trigger a call.
/// - Can be converted from [ObserverNotification]
/// - Can be converted from [ObserverTerminalNotification]
/// - Can be converted from [SubscriptionNotification]
/// - Can try to convert into a [ObserverNotification]
/// - Can try to convert into a [ObserverTerminalNotification]
/// - Can try to convert into a [SubscriptionNotification]
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubscriberNotification<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	Next(In),
	Error(InError),
	Complete,
	Unsubscribe,
}

impl<In, InError> SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	pub fn is_terminal(&self) -> bool {
		matches!(
			self,
			SubscriberNotification::Complete | SubscriberNotification::Error(_)
		)
	}

	#[inline]
	pub fn is_closing(&self) -> bool {
		self.is_terminal() || matches!(self, SubscriberNotification::Unsubscribe)
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

impl<In, InError> From<ObserverTerminalNotification<InError>>
	for SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: ObserverTerminalNotification<InError>) -> Self {
		match value {
			ObserverTerminalNotification::Error(error) => SubscriberNotification::Error(error),
			ObserverTerminalNotification::Complete => SubscriberNotification::Complete,
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

#[cfg(test)]
mod test {
	use super::*;

	mod push {
		use derive_where::derive_where;
		use rx_core_macro_subscriber_derive::RxSubscriber;

		use crate::{
			RxObserver, SubscriberPushNotificationExtention, SubscriptionLike, TeardownCollection,
		};

		use super::*;

		#[derive_where(Default)]
		#[derive(RxSubscriber)]
		#[_rx_core_common_crate(crate)]
		#[rx_in(In)]
		#[rx_in_error(InError)]
		struct MockSubscriber<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			next: Option<In>,
			error: Option<InError>,
			complete: bool,
			unsubscribed: bool,
		}

		impl<In, InError> RxObserver for MockSubscriber<In, InError>
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

		impl<In, InError> TeardownCollection for MockSubscriber<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			fn add_teardown(&mut self, _teardown: crate::Teardown) {
				unreachable!("not needed for these tests")
			}
		}

		impl<In, InError> SubscriptionLike for MockSubscriber<In, InError>
		where
			In: Signal,
			InError: Signal,
		{
			fn is_closed(&self) -> bool {
				self.unsubscribed
			}

			fn unsubscribe(&mut self) {
				self.unsubscribed = true;
			}
		}

		#[test]
		fn should_call_next_when_pushing_a_next_notification() {
			let mut subscriber = MockSubscriber::<usize, &'static str>::default();
			subscriber.push(SubscriberNotification::Next(1));
			assert_eq!(subscriber.next, Some(1));
		}

		#[test]
		fn should_call_error_when_pushing_an_error_notification() {
			let mut subscriber = MockSubscriber::<usize, &'static str>::default();
			let error = "error";
			subscriber.push(SubscriberNotification::Error(error));
			assert_eq!(subscriber.error, Some(error));
		}

		#[test]
		fn should_call_complete_when_pushing_a_complete_notification() {
			let mut subscriber = MockSubscriber::<usize, &'static str>::default();
			subscriber.push(SubscriberNotification::Complete);
			assert!(subscriber.complete);
		}

		#[test]
		fn should_call_unsubscribe_when_pushing_an_unsubscribe_notification() {
			let mut subscriber = MockSubscriber::<usize, &'static str>::default();
			subscriber.push(SubscriberNotification::Unsubscribe);
			assert!(subscriber.is_closed());
		}
	}

	mod observer_notification_conversion {
		use super::*;

		#[test]
		fn should_convert_next() {
			let subscriber_notification: SubscriberNotification<usize> =
				ObserverNotification::Next(1).into();
			assert_eq!(subscriber_notification, SubscriberNotification::Next(1));
		}

		#[test]
		fn should_convert_error() {
			let error = "error";
			let subscriber_notification: SubscriberNotification<usize, &'static str> =
				ObserverNotification::Error(error).into();
			assert_eq!(
				subscriber_notification,
				SubscriberNotification::Error(error)
			);
		}

		#[test]
		fn should_convert_complete() {
			let subscriber_notification: SubscriberNotification<usize, &'static str> =
				ObserverNotification::Complete.into();
			assert_eq!(subscriber_notification, SubscriberNotification::Complete);
		}
	}

	mod observer_terminal_notification_conversion {
		use super::*;

		#[test]
		fn should_convert_error() {
			let error = "error";
			let subscriber_notification: SubscriberNotification<usize, &'static str> =
				ObserverTerminalNotification::Error(error).into();
			assert_eq!(
				subscriber_notification,
				SubscriberNotification::Error(error)
			);
		}

		#[test]
		fn should_convert_complete() {
			let subscriber_notification: SubscriberNotification<usize, &'static str> =
				ObserverTerminalNotification::Complete.into();
			assert_eq!(subscriber_notification, SubscriberNotification::Complete);
		}
	}

	mod subscription_notification_conversion {
		use super::*;

		#[test]
		fn should_convert_unsubscribe() {
			let subscriber_notification: SubscriberNotification<usize> =
				SubscriptionNotification::Unsubscribe.into();
			assert_eq!(subscriber_notification, SubscriberNotification::Unsubscribe);
		}
	}

	mod is_terminal {
		use super::*;

		#[test]
		fn it_should_identify_terminal_notifications() {
			assert!(SubscriberNotification::<usize, &'static str>::Error("error").is_terminal());
			assert!(SubscriberNotification::<usize>::Complete.is_terminal());
			assert!(!SubscriberNotification::<usize>::Next(1).is_terminal());
			assert!(!SubscriberNotification::<usize>::Unsubscribe.is_terminal());
		}
	}

	mod is_closing {
		use super::*;

		#[test]
		fn it_should_identify_closing_notifications() {
			assert!(SubscriberNotification::<usize, &'static str>::Error("error").is_closing());
			assert!(SubscriberNotification::<usize>::Complete.is_closing());
			assert!(SubscriberNotification::<usize>::Unsubscribe.is_closing());
			assert!(!SubscriberNotification::<usize>::Next(1).is_closing());
		}
	}
}
