use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod when_emit_at_nth_is_non_zero {
	use super::*;

	#[test]
	fn should_emit_its_values_every_two_ticks_then_complete() {
		let mut mock_executor = MockExecutor::default();
		let scheduler = mock_executor.get_scheduler_handle();

		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut source = (1..=3).into_observable_on_every_nth_tick(
			OnTickObservableOptions {
				emit_at_every_nth_tick: 2,
				start_on_subscribe: true,
			},
			scheduler,
		);
		let mut subscription = source.subscribe(mock_destination);
		assert!(matches!(
			notification_collector.lock().nth_notification(0),
			SubscriberNotification::Next(1)
		));
		mock_executor.tick(Duration::from_millis(1));

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"should not have emitted after only one tick"
		);
		mock_executor.tick(Duration::from_millis(3));
		assert!(matches!(
			notification_collector.lock().nth_notification(1),
			SubscriberNotification::Next(2)
		));

		mock_executor.tick(Duration::from_millis(2));

		mock_executor.tick(Duration::from_millis(1));
		assert!(matches!(
			notification_collector.lock().nth_notification(2),
			SubscriberNotification::Next(3)
		));
		assert!(matches!(
			notification_collector.lock().nth_notification(3),
			SubscriberNotification::Complete
		));

		subscription.unsubscribe();

		notification_collector.lock().assert_notifications(
			"iterator_on_tick",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Complete,
			],
			true,
		);
		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
	}

	#[test]
	fn should_stop_early_if_downstream_gets_closed_early() {
		let mut mock_executor = MockExecutor::default();
		let scheduler = mock_executor.get_scheduler_handle();

		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let source = (1..=3).into_observable_on_every_nth_tick(
			OnTickObservableOptions {
				emit_at_every_nth_tick: 2,
				start_on_subscribe: true,
			},
			scheduler,
		);
		let mut subscription = source.take(2).subscribe(mock_destination);
		assert!(matches!(
			notification_collector.lock().nth_notification(0),
			SubscriberNotification::Next(1)
		));
		mock_executor.tick(Duration::from_millis(1));

		assert!(
			!notification_collector.lock().nth_notification_exists(2),
			"should not have emitted after only one tick"
		);
		mock_executor.tick(Duration::from_millis(3));
		assert!(matches!(
			notification_collector.lock().nth_notification(1),
			SubscriberNotification::Next(2)
		));

		mock_executor.tick(Duration::from_millis(2));
		mock_executor.tick(Duration::from_millis(1));

		notification_collector.lock().assert_notifications(
			"iterator_on_tick",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		subscription.unsubscribe();

		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
	}
}

mod when_emit_at_nth_is_zero {
	use super::*;

	#[test]
	fn should_immediately_emit_all_its_values_then_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let mut source = (1..=3).into_observable_on_every_nth_tick(
			OnTickObservableOptions {
				emit_at_every_nth_tick: 0, // This causes all values to be emitted immediately like a regular iterator
				start_on_subscribe: false,
			},
			scheduler,
		);
		let mut subscription = source.subscribe(mock_destination);

		notification_collector.lock().assert_notifications(
			"iterator_on_tick",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Next(3),
				SubscriberNotification::Complete,
			],
			true,
		);

		executor.tick(Duration::from_millis(1));

		subscription.unsubscribe();
		assert!(
			notification_collector
				.lock()
				.nothing_happened_after_closed(),
			"something happened after unsubscribe"
		);
	}

	#[test]
	fn should_not_finish_the_iterator_when_closed_early() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mock_destination = MockObserver::<i32>::default();
		let notification_collector = mock_destination.get_notification_collector();

		let tracked_iterator = TrackedIterator::new(1..=5);
		let tracked_data = tracked_iterator.get_tracking_data_ref();
		let mut source = tracked_iterator
			.into_observable_on_every_nth_tick(
				OnTickObservableOptions {
					emit_at_every_nth_tick: 0, // This causes all values to be emitted immediately like a regular iterator
					start_on_subscribe: false,
				},
				scheduler,
			)
			.take(2);
		let mut subscription = source.subscribe(mock_destination);

		notification_collector.lock().assert_notifications(
			"iterator_on_tick",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		assert!(!tracked_data.is_finished(0));

		subscription.unsubscribe();
	}
}

// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut finalized = SharedSubscription::default();
		let tracked_teardown = finalized.add_tracked_teardown("iterator_on_tick - source");

		let mut harness = TestHarness::<_, i32, Never>::new_with_source(
			"iterator_on_tick",
			(1..=3)
				.into_observable_on_every_nth_tick(
					OnTickObservableOptions {
						emit_at_every_nth_tick: 0,
						start_on_subscribe: false,
					},
					scheduler,
				)
				.finalize(move || finalized.unsubscribe()),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);

		harness.assert_terminal_notification(SubscriberNotification::Complete);

		tracked_teardown.assert_was_torn_down();
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let mut finalized = SharedSubscription::default();
		let tracked_teardown = finalized.add_tracked_teardown("iterator_on_tick - source");

		let mut harness = TestHarness::<_, i32, Never>::new_with_source(
			"iterator_on_tick",
			(1..=3)
				.into_observable_on_every_nth_tick(
					OnTickObservableOptions {
						emit_at_every_nth_tick: 2,
						start_on_subscribe: true,
					},
					scheduler,
				)
				.finalize(move || finalized.unsubscribe()),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		tracked_teardown.assert_was_torn_down();
	}
}
