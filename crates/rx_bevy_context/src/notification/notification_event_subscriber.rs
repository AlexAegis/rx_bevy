use bevy_derive::{Deref, DerefMut};
use bevy_ecs::event::Event;
use rx_core_traits::{
	Never, ObserverNotification, SignalBound, SubscriberNotification,
	SubscriberNotificationTryFromError,
};

use crate::BevySubscriptionContextProvider;

/// Since events are passed around as references and signals must be owned, we
/// can levarage the fact that these events are sent only once, and only to
/// one destination and let the `In` and `InError` signals be taken out of the
/// event.
#[derive(Event, Clone, Deref, DerefMut)]
pub struct SubscriberNotificationEvent<In, InError = Never>
where
	In: SignalBound,
	InError: SignalBound,
{
	notification: SubscriberNotification<In, InError, BevySubscriptionContextProvider>,
}

impl<In, InError> SubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[inline]
	pub fn is_unsubscribe(&self) -> bool {
		matches!(self.notification, SubscriberNotification::Unsubscribe)
	}
}

impl<In, InError> From<SubscriberNotification<In, InError, BevySubscriptionContextProvider>>
	for SubscriberNotificationEvent<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(
		notification: SubscriberNotification<In, InError, BevySubscriptionContextProvider>,
	) -> Self {
		Self { notification }
	}
}

impl<In, InError> From<SubscriberNotificationEvent<In, InError>>
	for SubscriberNotification<In, InError, BevySubscriptionContextProvider>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: SubscriberNotificationEvent<In, InError>) -> Self {
		value.notification
	}
}

impl<In, InError> TryFrom<SubscriberNotificationEvent<In, InError>>
	for ObserverNotification<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	type Error = SubscriberNotificationTryFromError;

	fn try_from(
		value: SubscriberNotificationEvent<In, InError>,
	) -> Result<Self, <ObserverNotification<In, InError> as TryFrom<SubscriberNotificationEvent<In, InError>>>::Error>{
		value.notification.try_into()
	}
}
