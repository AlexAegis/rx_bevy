use bevy::prelude::*;
use bevy_ecs::system::SystemState;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Event, Clone, Debug, PartialEq)]
struct TestEvent {
	pub value: usize,
}

mod when_used_as_a_component {
	use super::*;

	#[test]
	fn should_observe_bevy_events_and_emit_them_as_signals() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
		app.add_event::<TestEvent>();

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone())
				.into_component(),
		);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<TestEvent, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut()
			.trigger_targets(TestEvent { value: 0 }, event_target);
		app.world_mut()
			.trigger_targets(TestEvent { value: 1 }, event_target);

		subscription.unsubscribe();

		app.update(); // EntitySubscriptions unsubscribe by despawn

		app.world_mut()
			.trigger_targets(TestEvent { value: 2 }, event_target);

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent { value: 0 }),
				SubscriberNotification::Next(TestEvent { value: 1 }),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}

mod when_used_directly {
	use super::*;

	#[test]
	fn should_observe_bevy_events_and_emit_them_as_signals() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
		app.add_event::<TestEvent>();

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone());

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut()
			.trigger_targets(TestEvent { value: 0 }, event_target);
		app.world_mut()
			.trigger_targets(TestEvent { value: 1 }, event_target);

		subscription.unsubscribe();

		app.world_mut()
			.trigger_targets(TestEvent { value: 2 }, event_target);

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent { value: 0 }),
				SubscriberNotification::Next(TestEvent { value: 1 }),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_be_able_to_close_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
		app.add_event::<TestEvent>();

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone()).take(2);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut()
			.trigger_targets(TestEvent { value: 0 }, event_target);
		app.world_mut()
			.trigger_targets(TestEvent { value: 1 }, event_target);
		app.world_mut()
			.trigger_targets(TestEvent { value: 2 }, event_target);

		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[
				SubscriberNotification::Next(TestEvent { value: 0 }),
				SubscriberNotification::Next(TestEvent { value: 1 }),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update(); // To let the event observer_satellite_entity despawn

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);
	}

	#[test]
	fn should_be_able_to_close_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));
		app.add_event::<TestEvent>();

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();

		let mut event_observable =
			EventObservable::<TestEvent>::new(event_target, scheduler_handle.clone()).take(0);

		let destination = MockObserver::<TestEvent, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = event_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("event_observable");

		app.update();

		app.world_mut()
			.trigger_targets(TestEvent { value: 0 }, event_target);
		notification_collector.lock().assert_notifications(
			"event_observable",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update(); // To let the event observer_satellite_entity despawn

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);
	}
}
