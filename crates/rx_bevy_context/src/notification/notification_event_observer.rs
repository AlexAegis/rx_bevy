use bevy_ecs::event::Event;
use rx_core_traits::{Never, ObserverNotification, SignalBound, SubscriberNotification};

use crate::BevySubscriptionContextProvider;

/// # RxSignal (ObserverNotificationEvent)
///
/// While it would be much easier to just wrap ObserverNotification in a struct
/// instead of re-creating the enum, this is what is used directly by user, and
/// having the ability to match on this directly is worth more.
#[derive(Event, Clone, Debug)]
#[doc(alias = "ObserverNotificationEvent")]
pub enum RxSignal<In, InError = Never>
where
	In: SignalBound,
	InError: SignalBound,
{
	Next(In),
	Error(InError),
	Complete,
}

impl<In, InError> From<ObserverNotification<In, InError>> for RxSignal<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(notification: ObserverNotification<In, InError>) -> Self {
		match notification {
			ObserverNotification::Next(next) => RxSignal::Next(next),
			ObserverNotification::Error(error) => RxSignal::Error(error),
			ObserverNotification::Complete => RxSignal::Complete,
		}
	}
}

impl<In, InError> From<RxSignal<In, InError>> for ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		match value {
			RxSignal::Next(next) => ObserverNotification::Next(next),
			RxSignal::Error(error) => ObserverNotification::Error(error),
			RxSignal::Complete => ObserverNotification::Complete,
		}
	}
}

impl<In, InError> From<RxSignal<In, InError>>
	for SubscriberNotification<In, InError, BevySubscriptionContextProvider>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		let observer_notification: ObserverNotification<In, InError> = value.into();
		observer_notification.into()
	}
}
