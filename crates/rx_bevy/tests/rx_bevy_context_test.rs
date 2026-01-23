use bevy::prelude::*;
use bevy_ecs::world::DeferredWorld;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn it_should_be_able_to_send_observer_notifications() {
	let mut app = App::new();

	let observed_events = NotificationCollector::<usize>::default();
	let observed_events_clone = observed_events.clone();
	let target_entity = app
		.world_mut()
		.spawn_empty()
		.observe(move |observer_event: On<RxSignal<usize>>| {
			observed_events_clone
				.lock()
				.push(observer_event.signal().clone().into());
		})
		.id();

	app.update();
	let deferred_world = DeferredWorld::from(app.world_mut());
	let mut rx_context = RxBevyContextItem::from(deferred_world);
	rx_context.send_observer_notification(target_entity, ObserverNotification::<usize>::Complete);
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
			move |observer_event: On<SubscriberNotificationEvent<usize>>| {
				observed_events_clone
					.lock()
					.push(observer_event.signal().clone());
			},
		)
		.id();

	app.update();
	let deferred_world = DeferredWorld::from(app.world_mut());
	let mut rx_context = RxBevyContextItem::from(deferred_world);
	rx_context
		.send_subscriber_notification(target_entity, SubscriberNotification::<usize>::Complete);
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
		.observe(move |observer_event: On<SubscriptionNotificationEvent>| {
			observed_events_clone
				.lock()
				.push((*observer_event.signal()).into());
		})
		.id();

	app.update();
	let deferred_world = DeferredWorld::from(app.world_mut());
	let mut rx_context = RxBevyContextItem::from(deferred_world);
	rx_context.send_subscription_notification(target_entity, SubscriptionNotification::Unsubscribe);
	app.update();

	observed_events.lock().assert_notifications(
		"rx_bevy_context - subscriber notification",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}
