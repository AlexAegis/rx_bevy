use bevy_derive::Deref;
use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::Event,
};
use rx_core_common::{
	Never, ObserverNotification, Signal, SubscriberNotification,
	SubscriberToObserverNotificationConversionError,
};

/// Since events are passed around as references and signals must be owned, we
/// can levarage the fact that these events are sent only once, and only to
/// one destination and let the `In` and `InError` signals be taken out of the
/// event.
///
/// > This event is actually unused! It's here for the sake of completeness as
/// > other notification types have a corresponding event type, and third party
/// > implementations might want to leverage it.
// TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Clone, Deref)]
pub struct SubscriberNotificationEvent<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	// TODO(bevy-0.17): #[event_target]
	target: Entity,
	#[deref]
	notification: SubscriberNotification<In, InError>,
}

impl<In, InError> SubscriberNotificationEvent<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn signal(&self) -> &SubscriberNotification<In, InError> {
		&self.notification
	}
}

impl<In, InError> ContainsEntity for SubscriberNotificationEvent<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn entity(&self) -> Entity {
		self.target
	}
}

impl<In, InError> SubscriberNotificationEvent<In, InError>
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	pub fn from_notification(
		notification: SubscriberNotification<In, InError>,
		target: Entity,
	) -> Self {
		Self {
			notification,
			target,
		}
	}
}

impl<In, InError> From<SubscriberNotificationEvent<In, InError>>
	for SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: SubscriberNotificationEvent<In, InError>) -> Self {
		value.notification
	}
}

impl<In, InError> TryFrom<SubscriberNotificationEvent<In, InError>>
	for ObserverNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Error = SubscriberToObserverNotificationConversionError;

	fn try_from(
		value: SubscriberNotificationEvent<In, InError>,
	) -> Result<Self, <ObserverNotification<In, InError> as TryFrom<SubscriberNotificationEvent<In, InError>>>::Error>{
		value.notification.try_into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_should_create_an_event_with_unsubscribe_notification() {
		let event = SubscriberNotificationEvent::<usize, Never>::from_notification(
			SubscriberNotification::Unsubscribe,
			Entity::from_raw(1),
		);
		assert_eq!(event.entity(), Entity::from_raw(1));
		assert_eq!(*event, SubscriberNotification::Unsubscribe);
	}

	#[test]
	fn it_should_try_convert_to_observer_notification() {
		let event = SubscriberNotificationEvent::<usize, Never>::from_notification(
			SubscriberNotification::Next(42),
			Entity::from_raw(1),
		);
		let observer_notification: ObserverNotification<usize, Never> = event.try_into().unwrap();
		assert_eq!(observer_notification, ObserverNotification::Next(42));
	}

	#[test]
	fn it_should_fail_to_convert_unsubscribe_to_observer_notification() {
		let event = SubscriberNotificationEvent::<usize, Never>::from_notification(
			SubscriberNotification::Unsubscribe,
			Entity::from_raw(1),
		);
		let result: Result<ObserverNotification<usize, Never>, _> = event.try_into();
		assert_eq!(
			result.err().unwrap(),
			SubscriberToObserverNotificationConversionError::CannotReceiveUnsubscribe
		);
	}

	#[test]
	fn it_should_be_able_to_convert_a_subscriber_event() {
		let event = SubscriberNotificationEvent::<usize, &str>::from_notification(
			SubscriberNotification::Error("error"),
			Entity::from_raw(2),
		);
		let observer_notification: ObserverNotification<usize, &str> = event.try_into().unwrap();
		assert_eq!(observer_notification, ObserverNotification::Error("error"));
	}

	#[test]
	fn it_should_be_able_to_convert_from_a_subscriber_notification() {
		let entity = Entity::from_raw(10);
		let event = SubscriberNotificationEvent::<usize, Never>::from_notification(
			SubscriberNotification::Unsubscribe,
			entity,
		);
		let converted_notification: SubscriberNotification<usize, Never> = event.into();
		assert_eq!(converted_notification, SubscriberNotification::Unsubscribe);
	}
}
