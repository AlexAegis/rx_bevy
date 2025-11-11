use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	event::Event,
};
use rx_core_traits::SubscriptionNotification;
use thiserror::Error;

use crate::BevySubscriptionContextProvider;

// TODO(bevy-0.17): Use EntityEvent
#[derive(Event, Clone, Deref, DerefMut)]
pub struct SubscriptionNotificationEvent {
	// TODO(bevy-0.17): #[event_target]
	target: Entity,
	/// Subscription notifications must be consumable because they may own
	/// resources in the Add variant.
	#[deref]
	pub(crate) notification: Option<SubscriptionNotification<BevySubscriptionContextProvider>>,
}

impl ContainsEntity for SubscriptionNotificationEvent {
	fn entity(&self) -> Entity {
		self.target
	}
}

impl SubscriptionNotificationEvent {
	#[inline]
	pub fn from_notification(
		notification: SubscriptionNotification<BevySubscriptionContextProvider>,
		target: Entity,
	) -> Self {
		Self {
			notification: Some(notification),
			target,
		}
	}

	pub fn consume(
		&mut self,
	) -> Result<
		SubscriptionNotification<BevySubscriptionContextProvider>,
		SubscriptionNotificationEventConsumeError,
	> {
		self.notification
			.take()
			.ok_or(SubscriptionNotificationEventConsumeError)
	}

	pub fn retarget(
		&mut self,
		entity: Entity,
	) -> Result<Self, SubscriptionNotificationEventConsumeError> {
		Ok(Self::from_notification(self.consume()?, entity))
	}
}

#[derive(Error, Debug)]
#[error("Notification was already consumed!")]
pub struct SubscriptionNotificationEventConsumeError;

impl From<SubscriptionNotificationEvent>
	for SubscriptionNotification<BevySubscriptionContextProvider>
{
	fn from(mut event: SubscriptionNotificationEvent) -> Self {
		event.consume().unwrap()
	}
}
