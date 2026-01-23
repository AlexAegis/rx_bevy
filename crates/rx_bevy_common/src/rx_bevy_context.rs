use bevy_ecs::{entity::Entity, world::DeferredWorld};
use rx_core_common::{
	ObserverNotification, Signal, SubscriberNotification, SubscriptionNotification, WorkContext,
	WorkContextProvider,
};
use thiserror::Error;

use crate::{RxSignal, SubscriberNotificationEvent, SubscriptionNotificationEvent};

#[derive(Debug)]
pub struct RxBevyContext;

impl WorkContextProvider for RxBevyContext {
	type Item<'w> = RxBevyContextItem<'w>;
}

pub struct RxBevyContextItem<'w> {
	pub deferred_world: DeferredWorld<'w>,
}

impl<'w> WorkContext<'w> for RxBevyContextItem<'w> {}

#[derive(Error, Debug)]
pub enum ContextGetSubscriptionsErasedScheduleError {
	#[error(
		"Attempted to create a ProxySubscription with an incomplete Context! It does not contain a parent subscription entity!"
	)]
	ContextDoesNotHaveASubscritpionEntity,
	#[error("Subscription Entity {0} should have an ErasedSubscriptionSchedule!")]
	SubscriptionEntityDoesNotHaveAnErasedSubscriptionSchedule(Entity),
}

impl<'w> RxBevyContextItem<'w> {
	#[inline]
	pub fn reborrow(&mut self) -> RxBevyContextItem<'_> {
		RxBevyContextItem {
			deferred_world: self.deferred_world.reborrow(),
		}
	}

	pub fn send_observer_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: ObserverNotification<In, InError>,
	) where
		In: Signal,
		InError: Signal,
	{
		let notification_event = RxSignal::<In, InError>::from_notification(notification, target);
		self.deferred_world.commands().trigger(notification_event);
	}

	pub fn send_subscriber_notification<In, InError>(
		&mut self,
		target: Entity,
		notification: SubscriberNotification<In, InError>,
	) where
		In: Signal,
		InError: Signal,
	{
		let notification_event =
			SubscriberNotificationEvent::<In, InError>::from_notification(notification, target);
		self.deferred_world.commands().trigger(notification_event);
	}

	pub fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification,
	) {
		let notification_event =
			SubscriptionNotificationEvent::from_notification(notification, target);
		self.deferred_world.commands().trigger(notification_event);
	}
}

#[derive(Error, Debug)]
pub enum ContextAccessError {
	#[error("Tried to get {0}. But it does not exist on entity {1}.")]
	NotAnObservable(String, Entity),
}

impl<'w> From<DeferredWorld<'w>> for RxBevyContextItem<'w> {
	fn from(deferred_world: DeferredWorld<'w>) -> Self {
		RxBevyContextItem { deferred_world }
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use bevy_app::App;
	use bevy_ecs::world::DeferredWorld;

	#[test]
	fn it_should_be_reborrowable() {
		let mut app = App::new();
		let entity = {
			let deferred_world = DeferredWorld::from(app.world_mut());
			let mut rx_context = RxBevyContextItem::from(deferred_world);
			rx_context.deferred_world.commands().spawn_empty().id()
		};
		app.update();
		let deferred_world = DeferredWorld::from(app.world_mut());
		let mut rx_context = RxBevyContextItem::from(deferred_world);
		let rx_context_reborrowed = rx_context.reborrow();
		assert!(
			rx_context_reborrowed
				.deferred_world
				.get_entity(entity)
				.is_ok()
		);
	}
}
