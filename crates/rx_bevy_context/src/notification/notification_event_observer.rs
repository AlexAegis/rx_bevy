use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::EntityEvent,
};
use rx_core_traits::{Never, ObserverNotification, SignalBound, SubscriberNotification};

use crate::BevySubscriptionContextProvider;

/// # RxSignal (ObserverNotificationEvent)
///
#[derive(EntityEvent, Clone, Debug)]
#[doc(alias = "ObserverNotificationEvent")]
pub struct RxSignal<In, InError = Never>
where
	In: SignalBound,
	InError: SignalBound,
{
	#[event_target]
	target: Entity,
	notification: ObserverNotification<In, InError>,
}

impl<In, InError> ContainsEntity for RxSignal<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	fn entity(&self) -> Entity {
		self.target
	}
}

impl<In, InError> RxSignal<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
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
	In: SignalBound,
	InError: SignalBound,
{
	fn from(value: RxSignal<In, InError>) -> Self {
		value.notification
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
