use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::{MockObserver, NotificationCollector, TrackTeardownExtension};

#[test]
fn commands_with_observable_spawns_and_despawns_subscription_entity() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let scheduler_handle = {
		let schedule = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		schedule.handle()
	};

	let destination = MockObserver::<usize>::default();
	let notifications: NotificationCollector<usize, _> = destination.get_notification_collector();

	let mut commands = app.world_mut().commands();
	let mut observable_with_commands =
		commands.with_observable(JustObservable::new(3_usize), scheduler_handle);

	let subscription_entity = observable_with_commands.subscribe(destination).entity();

	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_err(),
		"subscription entity should be despawned after the observable completes immediately",
	);

	let notifications = notifications.lock();
	notifications.assert_notifications(
		"commands_with_observable - immediate completion",
		0,
		[
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn observable_with_commands_spawns_and_despawns_subscription_entity() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let scheduler_handle = {
		let schedule = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		schedule.handle()
	};

	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();
	let mut subject = PublishSubject::<usize>::default();

	let commands = app.world_mut().commands();
	let mut observable_with_commands = subject.clone().with_commands(commands, scheduler_handle);

	let mut subscription = observable_with_commands.subscribe(destination);
	let (teardown, tracker) = Teardown::tracked("observable_with_commands_teardown_on_complete");
	subscription.add(teardown);
	let subscription_entity = subscription.entity();

	subject.next(42);
	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_ok(),
		"subscription entity should exist while the observable is active",
	);

	subject.complete();
	app.update();

	tracker.assert_was_torn_down();
	assert!(
		app.world().get_entity(subscription_entity).is_err(),
		"subscription entity should despawn after the observable completes",
	);

	notifications.lock().assert_notifications(
		"observable_with_commands - completion",
		0,
		[
			SubscriberNotification::Next(42),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn commands_with_observable_despawns_after_manual_unsubscribe() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let scheduler_handle = {
		let schedule = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		schedule.handle()
	};

	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();
	let mut subject = PublishSubject::<usize>::default();

	let mut commands = app.world_mut().commands();
	let mut observable_with_commands = commands.with_observable(subject.clone(), scheduler_handle);

	let mut subscription = observable_with_commands.subscribe(destination);
	let subscription_entity = subscription.entity();

	subject.next(1);
	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_ok(),
		"subscription entity should exist while the observable is active",
	);

	subject.next(2);
	app.update();

	subscription.unsubscribe();
	app.update();

	assert!(
		app.world().get_entity(subscription_entity).is_err(),
		"subscription entity should despawn after manual unsubscribe",
	);

	notifications.lock().assert_notifications(
		"commands_with_observable - manual unsubscribe",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn commands_with_observable_executes_teardowns_on_unsubscribe() {
	let mut app = App::new();
	app.init_resource::<Time<Virtual>>();
	app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

	let scheduler_handle = {
		let schedule = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
			.get_mut(app.world_mut());
		schedule.handle()
	};

	let destination = MockObserver::<usize>::default();
	let notifications = destination.get_notification_collector();
	let mut subject = PublishSubject::<usize>::default();

	let mut commands = app.world_mut().commands();
	let mut observable_with_commands = commands.with_observable(subject.clone(), scheduler_handle);

	let mut subscription = observable_with_commands.subscribe(destination);
	let (teardown, tracker) = Teardown::tracked("commands_with_observable_teardown");
	subscription.add(teardown);

	let subscription_entity = subscription.entity();

	subject.next(9);
	app.update();

	subscription.unsubscribe();
	app.update();

	tracker.assert_was_torn_down();
	assert!(
		app.world().get_entity(subscription_entity).is_err(),
		"subscription entity should despawn after unsubscribe",
	);

	notifications.lock().assert_notifications(
		"commands_with_observable - teardown on unsubscribe",
		0,
		[
			SubscriberNotification::Next(9),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
