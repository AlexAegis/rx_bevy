use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use rx_bevy::prelude::*;
use rx_core_testing::prelude::*;

#[derive(Event, Clone, Debug, PartialEq)]
struct TestMessage {
	pub value: usize,
}

mod when_used_as_a_component {
	use super::*;

	#[test]
	fn should_observe_messages() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_event::<TestMessage>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let observable_entity = app
			.world_mut()
			.commands()
			.spawn(MessageObservable::<TestMessage>::new(scheduler_handle.clone()).into_component())
			.id();

		let destination = MockObserver::<TestMessage, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = app
			.world_mut()
			.commands()
			.entity(observable_entity)
			.as_observable::<TestMessage, Never>(scheduler_handle)
			.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("message_observable");

		app.update();
		app.world_mut().send_event(TestMessage { value: 0 });

		subscription.unsubscribe();

		app.update();

		notification_collector.lock().assert_notifications(
			"message_observable",
			0,
			[
				SubscriberNotification::Next(TestMessage { value: 0 }),
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
		app.add_event::<TestMessage>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut message_observable =
			MessageObservable::<TestMessage>::new(scheduler_handle.clone());

		let destination = MockObserver::<TestMessage, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = message_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("message_observable");

		app.world_mut().send_event(TestMessage { value: 0 });

		app.update();

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"message_observable",
			0,
			[
				SubscriberNotification::Next(TestMessage { value: 0 }),
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
		app.add_event::<TestMessage>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut message_observable =
			MessageObservable::<TestMessage>::new(scheduler_handle.clone());

		let destination = MockObserver::<TestMessage, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = message_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("message_observable");

		app.world_mut().send_event(TestMessage { value: 0 });
		app.world_mut().send_event(TestMessage { value: 1 });

		app.update();
		subscription.unsubscribe();

		app.world_mut().send_event(TestMessage { value: 2 });

		app.update();

		notification_collector.lock().assert_notifications(
			"message_observable",
			0,
			[
				SubscriberNotification::Next(TestMessage { value: 0 }),
				SubscriberNotification::Next(TestMessage { value: 1 }),
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
				"message_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_early() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_event::<TestMessage>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut message_observable =
			MessageObservable::<TestMessage>::new(scheduler_handle.clone()).take(2);

		let destination = MockObserver::<TestMessage, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = message_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("message_observable");

		app.update();
		app.world_mut().send_event(TestMessage { value: 0 });
		app.world_mut().send_event(TestMessage { value: 1 });
		app.update();
		app.world_mut().send_event(TestMessage { value: 2 });

		notification_collector.lock().assert_notifications(
			"message_observable",
			0,
			[
				SubscriberNotification::Next(TestMessage { value: 0 }),
				SubscriberNotification::Next(TestMessage { value: 1 }),
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
				"message_observable - rx_verify_no_new_notification_after_closed",
				2,
			);
	}

	#[test]
	fn rx_contract_closed_if_downstream_closes_immediately() {
		let mut app = App::new();
		app.init_resource::<Time<Virtual>>();
		app.add_event::<TestMessage>();
		app.add_plugins((RxPlugin, RxSchedulerPlugin::<Update, Virtual>::default()));

		let scheduler_handle = {
			let scheduler = SystemState::<RxSchedule<Update, Virtual>>::new(app.world_mut())
				.get_mut(app.world_mut());
			scheduler.handle()
		};

		let mut message_observable =
			MessageObservable::<TestMessage>::new(scheduler_handle.clone()).take(0);

		let destination = MockObserver::<TestMessage, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = message_observable.subscribe(destination);
		let tracked_teardown = subscription.add_tracked_teardown("message_observable");

		notification_collector.lock().assert_notifications(
			"message_observable",
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
				"message_observable - rx_verify_no_new_notification_after_closed",
				0,
			);
	}
}
