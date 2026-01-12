use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

mod when_used_as_a_component {
	use super::*;

	#[test]
	fn should_observe_just_pressed_events() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::JustPressed,
				},
				scheduler_handle.clone(),
			)
			.into_component(),
		);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<KeyCode, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::Space);
		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::Space);
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::Space),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_observe_just_released_events() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::JustReleased,
				},
				scheduler_handle.clone(),
			)
			.into_component(),
		);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<KeyCode, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::Space);
		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.release(KeyCode::Space);
		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_released(KeyCode::Space);
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::Space),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_observe_events_while_pressed() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let event_target = app.world_mut().commands().spawn_empty().id();
		app.world_mut().commands().entity(event_target).insert(
			KeyboardObservable::new(
				KeyboardObservableOptions {
					emit: KeyboardObservableEmit::WhilePressed,
				},
				scheduler_handle.clone(),
			)
			.into_component(),
		);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(event_target)
			.as_observable::<KeyCode, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::Space);

		app.update();
		app.update();
		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.release(KeyCode::Space);
		app.update();

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::Space),
				SubscriberNotification::Next(KeyCode::Space),
				SubscriberNotification::Next(KeyCode::Space),
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
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut keyboard_observable = KeyboardObservable::new(
			KeyboardObservableOptions {
				emit: KeyboardObservableEmit::JustPressed,
			},
			scheduler_handle.clone(),
		);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = keyboard_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::Space);

		app.update();

		subscription.unsubscribe();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::Escape);

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::Space),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());
	}
}

/// Non Applicable:
/// - rx_contract_closed_after_complete - Can't Complete
/// - rx_contract_closed_after_error - Can't Error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut keyboard_observable = KeyboardObservable::new(
			KeyboardObservableOptions {
				emit: KeyboardObservableEmit::JustPressed,
			},
			scheduler_handle.clone(),
		);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = keyboard_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyA);

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::KeyA);
		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyB);

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::KeyB);

		subscription.unsubscribe();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyC);

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::KeyA),
				SubscriberNotification::Next(KeyCode::KeyB),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		app.update();

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"keyboard_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut keyboard_observable = KeyboardObservable::new(
			KeyboardObservableOptions {
				emit: KeyboardObservableEmit::JustPressed,
			},
			scheduler_handle.clone(),
		)
		.take(2);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = keyboard_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyA);

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::KeyA);
		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyB);

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::KeyB);

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.press(KeyCode::KeyC);

		app.update();

		app.world_mut()
			.resource_mut::<ButtonInput<KeyCode>>()
			.clear_just_pressed(KeyCode::KeyC);

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[
				SubscriberNotification::Next(KeyCode::KeyA),
				SubscriberNotification::Next(KeyCode::KeyB),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"keyboard_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.init_resource::<ButtonInput<KeyCode>>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut keyboard_observable = KeyboardObservable::new(
			KeyboardObservableOptions {
				emit: KeyboardObservableEmit::JustPressed,
			},
			scheduler_handle.clone(),
		)
		.take(0);

		let destination = MockObserver::<KeyCode, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = keyboard_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("keyboard_observable");

		notification_collector.lock().assert_notifications(
			"keyboard_observable",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		assert!(subscription.is_closed());

		assert!(
			app.world()
				.resource::<RxBevyExecutor<Update, Virtual>>()
				.is_empty(),
			"No work should remain in the executor"
		);

		subscription.unsubscribe();
		notification_collector
			.lock()
			.assert_nth_notification_is_last(
				"keyboard_observable - rx_verify_no_new_notification_after_closed",
				0,
			);
	}
}
