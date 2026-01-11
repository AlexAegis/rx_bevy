use bevy_derive::Deref;
use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::Event,
};
use rx_core_common::{Never, ObserverNotification, Signal, SubscriberNotification};

/// # RxSignal (ObserverNotificationEvent)
///  TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Clone, Debug, Deref)]
#[doc(alias = "ObserverNotificationEvent")]
pub struct RxSignal<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	// TODO(bevy-0.17): #[event_target]
	target: Entity,
	#[deref]
	notification: ObserverNotification<In, InError>,
}

impl<In, InError> ContainsEntity for RxSignal<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn entity(&self) -> Entity {
		self.target
	}
}

impl<In, InError> RxSignal<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new_next(next: In, destination: Entity) -> Self {
		Self {
			target: destination,
			notification: ObserverNotification::Next(next),
		}
	}

	pub fn new_error(error: InError, destination: Entity) -> Self {
		Self {
			target: destination,
			notification: ObserverNotification::Error(error),
		}
	}

	pub fn new_complete(destination: Entity) -> Self {
		Self {
			target: destination,
			notification: ObserverNotification::Complete,
		}
	}

	pub fn signal(&self) -> &ObserverNotification<In, InError> {
		&self.notification
	}

	pub fn from_notification(
		notification: ObserverNotification<In, InError>,
		target: Entity,
	) -> Self {
		Self {
			target,
			notification,
		}
	}
}

impl<In, InError> From<RxSignal<In, InError>> for ObserverNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		value.notification
	}
}

impl<In, InError> From<RxSignal<In, InError>> for SubscriberNotification<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		let observer_notification: ObserverNotification<In, InError> = value.into();
		observer_notification.into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	mod new_next {
		use super::*;

		#[test]
		fn it_should_create_an_event_with_a_next_notification() {
			let event = RxSignal::<usize, Never>::new_next(42, Entity::from_raw(1));
			assert_eq!(event.entity(), Entity::from_raw(1));
			assert_eq!(event.signal(), &ObserverNotification::Next(42));
		}
	}

	mod new_error {
		use super::*;

		#[test]
		fn it_should_create_an_event_with_an_error_notification() {
			let event = RxSignal::<usize, &str>::new_error("error", Entity::from_raw(2));
			assert_eq!(event.entity(), Entity::from_raw(2));
			assert_eq!(*event, ObserverNotification::Error("error"));
		}
	}

	mod new_complete {
		use super::*;

		#[test]
		fn it_should_create_an_event_with_a_complete_notification() {
			let event = RxSignal::<usize, Never>::new_complete(Entity::from_raw(3));
			assert_eq!(event.entity(), Entity::from_raw(3));
			assert_eq!(*event, ObserverNotification::Complete);
		}
	}
}
