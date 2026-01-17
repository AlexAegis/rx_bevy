use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::{Never, SubscriberNotification};
use rx_core_testing::prelude::*;

#[path = "./utilities.rs"]
mod utilities;

use utilities::*;

#[test]
fn entity_commands_can_subscribe_to_another_entity_observable() {
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

	let subscription_entity = app
		.world_mut()
		.commands()
		.entity(destination_entity)
		.subscribes_to_observable_entity::<usize, Never>(observable_entity, scheduler_handle);

	app.update();

	subject.next(1);
	app.update();
	subject.next(2);
	app.update();

	app.world_mut().despawn(subscription_entity);
	app.update();

	subject.next(3);
	app.update();

	notifications.lock().assert_notifications(
		"entity_commands_subscribes_to_observable_entity",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);
}

#[test]
fn entity_commands_can_subscribe_to_immediately_completing_observable() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let notifications = NotificationCollector::default();

	let observable_entity = app
		.world_mut()
		.spawn(JustObservable::new(5_usize).into_component())
		.id();

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

	let subscription_entity = app
		.world_mut()
		.commands()
		.entity(destination_entity)
		.subscribes_to_observable_entity::<usize, Never>(observable_entity, scheduler_handle);

	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_err(),
		"subscription entity should be cleaned up for immediately completing observables",
	);

	notifications.lock().assert_notifications(
		"entity_commands_subscribes_to_immediate_observable",
		0,
		[
			SubscriberNotification::Next(5),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn entity_commands_can_subscribe_destination_directly() {
	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();

	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let mut subject = PublishSubject::<usize>::default();
	let observable_entity = app.world_mut().spawn(subject.clone().into_component()).id();

	let subscription_entity = app
		.world_mut()
		.commands()
		.entity(observable_entity)
		.subscribe_destination(destination);

	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_ok(),
		"subscription entity should be spawned when subscribing with destination",
	);

	subject.next(1);
	subject.next(2);

	app.update();

	app.world_mut().despawn(subscription_entity);

	subject.next(99);

	app.update();

	notifications.lock().assert_notifications(
		"entity_commands_subscribe_destination",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
