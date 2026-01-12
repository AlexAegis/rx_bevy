use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::{Never, SubscriberNotification};
use rx_core_testing::prelude::*;

#[path = "./utilities.rs"]
mod utilities;

use utilities::*;

#[test]
fn signals_should_reach_the_destination_and_close_on_error() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let notifications = NotificationCollector::default();

	let destination_entity = app
		.world_mut()
		.spawn_empty()
		.observe(collect_notifications_into::<usize, TestError>(
			notifications.clone(),
		))
		.id();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let mut destination =
		EntityDestination::<usize, TestError>::new(destination_entity, scheduler_handle).upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.error(TestError);

	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	notifications.lock().assert_notifications(
		"entity_destination_error",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Error(TestError),
		],
		true,
	);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}

#[test]
fn signals_should_reach_the_destination_and_close_on_complete() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let notifications = NotificationCollector::default();

	let destination_entity = app
		.world_mut()
		.spawn_empty()
		.observe(collect_notifications_into::<usize, Never>(
			notifications.clone(),
		))
		.id();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let mut destination =
		EntityDestination::<usize>::new(destination_entity, scheduler_handle).upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.complete();
	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	notifications.lock().assert_notifications(
		"entity_destination_complete",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
		],
		true,
	);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}

#[test]
fn signals_should_reach_the_destination_and_close_on_unsubscribe() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let notifications = NotificationCollector::default();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let destination_entity = app
		.world_mut()
		.spawn_empty()
		.observe(collect_notifications_into::<usize, Never>(
			notifications.clone(),
		))
		.id();

	let mut destination =
		EntityDestination::<usize>::new(destination_entity, scheduler_handle).upgrade();
	let tracked_teardown = destination.add_tracked_teardown("entity_destination");
	destination.next(1);
	destination.next(2);
	destination.unsubscribe();
	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	notifications.lock().assert_notifications(
		"entity_destination_complete",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	tracked_teardown.assert_was_torn_down();
	assert!(destination.is_closed(), "rx_verify_closed");
}

#[test]
fn despawned_destination_can_no_longer_receive_notifications() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let notifications = NotificationCollector::default();

	let mut subject = PublishSubject::<usize>::default();
	let observable_entity = app.world_mut().spawn(subject.clone().into_component()).id();
	let destination_entity = app
		.world_mut()
		.spawn_empty()
		.observe(collect_notifications_into::<usize, Never>(
			notifications.clone(),
		))
		.id();

	let scheduler_handle = {
		let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		scheduler.handle()
	};

	let subscription_entity = app.world_mut().commands().subscribe(
		observable_entity,
		EntityDestination::<usize>::new(destination_entity, scheduler_handle),
	);

	app.update();

	subject.next(1);
	subject.next(2);

	app.update();

	app.world_mut().despawn(destination_entity);

	app.update();

	subject.next(3);

	app.update();

	// Note that these were converted from ObserverNotifications, Unsubscribe can't show up here.
	notifications.lock().assert_notifications(
		"entity_destination_despawned_destination",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	let _subscription = app
		.world_mut()
		.entity(subscription_entity)
		.get::<SubscriptionComponent>();
}
