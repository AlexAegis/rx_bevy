use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::Event,
};
use rx_core_common::{Never, ObserverNotification, Signal, SubscriberNotification};

/// # RxSignal (ObserverNotificationEvent)
///  TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Clone, Debug)]
#[doc(alias = "ObserverNotificationEvent")]
pub struct RxSignal<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	// TODO(bevy-0.17): #[event_target]
	destination: Entity,
	notification: ObserverNotification<In, InError>,
}

impl<In, InError> ContainsEntity for RxSignal<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn entity(&self) -> Entity {
		self.destination
	}
}

impl<In, InError> RxSignal<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new_next(next: In, destination: Entity) -> Self {
		Self {
			destination,
			notification: ObserverNotification::Next(next),
		}
	}

	pub fn new_error(error: InError, destination: Entity) -> Self {
		Self {
			destination,
			notification: ObserverNotification::Error(error),
		}
	}

	pub fn new_complete(destination: Entity) -> Self {
		Self {
			destination,
			notification: ObserverNotification::Complete,
		}
	}

	pub fn signal(&self) -> &ObserverNotification<In, InError> {
		&self.notification
	}

	pub fn from_notification(
		notification: ObserverNotification<In, InError>,
		destination: Entity,
	) -> Self {
		Self {
			destination,
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
