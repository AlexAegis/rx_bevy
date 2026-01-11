use bevy_ecs::{
	entity::{ContainsEntity, Entity},
	world::DeferredWorld,
};
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
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
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
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
	}

	pub fn send_subscription_notification(
		&mut self,
		target: Entity,
		notification: SubscriptionNotification,
	) {
		let notification_event =
			SubscriptionNotificationEvent::from_notification(notification, target);
		// TODO(bevy-0.17): Use this
		// self.deferred_world.commands().trigger(notification_event);
		let target = notification_event.entity();
		self.deferred_world
			.commands()
			.trigger_targets(notification_event, target);
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
	use bevy_ecs::{observer::Trigger, world::DeferredWorld};
	use rx_core_testing::NotificationCollector;

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

	mod notifications {
		use super::*;

		#[test]
		fn it_should_be_able_to_send_observer_notifications() {
			let mut app = App::new();

			let observed_events = NotificationCollector::<usize>::default();
			let observed_events_clone = observed_events.clone();
			let target_entity = app
				.world_mut()
				.spawn_empty()
				.observe(move |observer_event: Trigger<RxSignal<usize>>| {
					observed_events_clone
						.lock()
						.push(observer_event.signal().clone().into());
				})
				.id();

			app.update();
			let deferred_world = DeferredWorld::from(app.world_mut());
			let mut rx_context = RxBevyContextItem::from(deferred_world);
			rx_context
				.send_observer_notification(target_entity, ObserverNotification::<usize>::Complete);
			app.update();

			observed_events.lock().assert_notifications(
				"rx_bevy_context - observer notification (rx_signal)",
				0,
				[SubscriberNotification::Complete],
				true,
			);
		}

		#[test]
		fn it_should_be_able_to_send_subscriber_notifications() {
			let mut app = App::new();

			let observed_events = NotificationCollector::<usize>::default();
			let observed_events_clone = observed_events.clone();
			let target_entity = app
				.world_mut()
				.spawn_empty()
				.observe(
					move |observer_event: Trigger<SubscriberNotificationEvent<usize>>| {
						observed_events_clone
							.lock()
							.push(observer_event.signal().clone());
					},
				)
				.id();

			app.update();
			let deferred_world = DeferredWorld::from(app.world_mut());
			let mut rx_context = RxBevyContextItem::from(deferred_world);
			rx_context.send_subscriber_notification(
				target_entity,
				SubscriberNotification::<usize>::Complete,
			);
			app.update();

			observed_events.lock().assert_notifications(
				"rx_bevy_context - subscriber notification",
				0,
				[SubscriberNotification::Complete],
				true,
			);
		}

		#[test]
		fn it_should_be_able_to_send_subscription_notifications() {
			let mut app = App::new();

			let observed_events = NotificationCollector::<usize>::default();
			let observed_events_clone = observed_events.clone();
			let target_entity = app
				.world_mut()
				.spawn_empty()
				.observe(
					move |observer_event: Trigger<SubscriptionNotificationEvent>| {
						observed_events_clone
							.lock()
							.push((*observer_event.signal()).into());
					},
				)
				.id();

			app.update();
			let deferred_world = DeferredWorld::from(app.world_mut());
			let mut rx_context = RxBevyContextItem::from(deferred_world);
			rx_context.send_subscription_notification(
				target_entity,
				SubscriptionNotification::Unsubscribe,
			);
			app.update();

			observed_events.lock().assert_notifications(
				"rx_bevy_context - subscriber notification",
				0,
				[SubscriberNotification::Unsubscribe],
				true,
			);
		}
	}
}
