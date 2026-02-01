use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::{Never, Observable};
use rx_core_testing::prelude::*;

mod when_leading_true {
	use super::*;

	#[test]
	fn should_emit_immediately_then_suppress_until_duration() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::LeadingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 0);

		executor.tick(Duration::from_millis(500));
		source.next(2);
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 0);

		executor.tick(Duration::from_millis(500));
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 0);

		source.next(3);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Next(3)],
			true,
		);

		executor.tick(Duration::from_millis(1000));
		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
	}
}

mod when_trailing_false {
	use super::*;

	#[test]
	fn should_not_emit_trailing_value_when_disabled() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::LeadingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		source.complete();
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);
		source.next(2);
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 1);

		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
	}

	#[test]
	fn should_ignore_values_during_throttle_when_trailing_disabled() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::LeadingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		executor.tick(Duration::from_millis(500));
		source.next(2);
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 0);

		executor.tick(Duration::from_millis(500));
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 0);

		source.next(3);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Next(3)],
			true,
		);

		executor.tick(Duration::from_millis(1000));
		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
	}
}

mod when_leading_false {
	use super::*;

	#[test]
	fn should_delay_first_emission_when_leading_disabled() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::TrailingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);

		notification_collector
			.lock()
			.assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(1000));
		executor.tick(Duration::from_millis(0));

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
	}
}

mod when_trailing_true {
	use super::*;

	#[test]
	fn should_emit_trailing_value_when_enabled() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let mut subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::TrailingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		notification_collector
			.lock()
			.assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(500));
		source.next(2);
		notification_collector
			.lock()
			.assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(2)],
			true,
		);
		executor.tick(Duration::from_millis(0));

		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
	}

	#[test]
	fn should_complete_after_trailing_emission_when_pending() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::TrailingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		source.complete();

		notification_collector
			.lock()
			.assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);
		executor.tick(Duration::from_millis(0));

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_unsubscribe_after_trailing_emission_when_pending() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::TrailingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		source.unsubscribe();

		notification_collector
			.lock()
			.assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
		executor.tick(Duration::from_millis(0));

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_ignore_next_after_unsubscribe_until_trailing_emitted() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"throttle_time",
		);

		let mut observable = harness.create_harness_observable().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(1000))
				.with_output(ThrottleOutputBehavior::TrailingOnly),
			scheduler.clone(),
		);
		let destination = harness.create_harness_destination(None);
		harness.register_subscription(observable.subscribe(destination));

		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		harness.source().next(2);

		harness.notifications().assert_is_empty("throttle_time");

		executor.tick(Duration::from_millis(1000));

		harness.notifications().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
		executor.tick(Duration::from_millis(0));

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(harness.is_subscription_closed());
	}

	#[test]
	fn should_error_immediately_and_cancel_pending_throttled_values() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::TrailingOnly),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);

		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
		assert!(subscription.is_closed());
	}
}

mod when_leading_and_trailing_true {
	use super::*;

	#[test]
	fn should_throttle_interval_source() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();

		let mut subscription = interval(
			IntervalObservableOptions {
				duration: Duration::from_millis(1),
				max_emissions_per_tick: 1000,
				..Default::default()
			},
			scheduler.clone(),
		)
		.pipe(ThrottleTimeOperator::new(
			Duration::from_millis(500),
			scheduler,
		))
		.subscribe(destination);

		executor.tick(Duration::from_millis(100));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(0)],
			true,
		);

		for _ in 0..4 {
			executor.tick(Duration::from_millis(100));
		}
		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Next(499)],
			true,
		);

		for _ in 0..5 {
			executor.tick(Duration::from_millis(100));
		}
		notification_collector.lock().assert_notifications(
			"throttle_time",
			2,
			[SubscriberNotification::Next(999)],
			true,
		);

		subscription.unsubscribe();
	}

	#[test]
	fn should_restart_throttling_before_duration_after_trailing() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();
		let interval_scheduler = scheduler.clone();

		let mut source = PublishSubject::<(), Never>::default();
		let mut subscription = source
			.clone()
			.switch_map(
				move |_| {
					interval(
						IntervalObservableOptions {
							duration: Duration::from_millis(500),
							max_emissions_per_tick: 10,
							start_on_subscribe: false,
						},
						interval_scheduler.clone(),
					)
					.take(3)
				},
				Never::map_into(),
			)
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(500)),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(());
		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(0)],
			true,
		);

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Next(1)],
			true,
		);

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			2,
			[SubscriberNotification::Next(2)],
			true,
		);

		executor.tick(Duration::from_millis(200));
		source.next(());
		notification_collector
			.lock()
			.assert_nth_notification_is_last("throttle_time", 2);

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			3,
			[SubscriberNotification::Next(0)],
			true,
		);

		subscription.unsubscribe();
	}

	#[test]
	fn should_allow_new_window_shortly_after_trailing_with_switch_map() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, Never>::default();
		let notification_collector = destination.get_notification_collector();
		let interval_scheduler = scheduler.clone();

		let mut source = PublishSubject::<(), Never>::default();
		let mut subscription = source
			.clone()
			.switch_map(
				move |_| {
					interval(
						IntervalObservableOptions {
							duration: Duration::from_millis(500),
							max_emissions_per_tick: 10,
							start_on_subscribe: true,
						},
						interval_scheduler.clone(),
					)
					.take(3)
				},
				Never::map_into(),
			)
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(500)),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(());
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(0)],
			true,
		);

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Next(1)],
			true,
		);

		executor.tick(Duration::from_millis(500));
		notification_collector.lock().assert_notifications(
			"throttle_time",
			2,
			[SubscriberNotification::Next(2)],
			true,
		);

		executor.tick(Duration::from_millis(200));
		source.next(());
		notification_collector.lock().assert_notifications(
			"throttle_time",
			3,
			[SubscriberNotification::Next(0)],
			true,
		);

		subscription.unsubscribe();
	}

	#[test]
	fn should_immediately_complete_when_no_pending_values_exist() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(ThrottleTimeOptions::default(), scheduler.clone())
			.subscribe(destination);

		source.complete();

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_complete_when_throttling_without_trailing_value() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000))
					.with_output(ThrottleOutputBehavior::LeadingAndTrailing),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.next(1);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		source.complete();

		notification_collector.lock().assert_notifications(
			"throttle_time",
			1,
			[SubscriberNotification::Complete],
			true,
		);
		executor.tick(Duration::from_millis(0));

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_immediately_unsubscribe_when_no_pending_values_exist() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.throttle_time(
				ThrottleTimeOptions::new(Duration::from_millis(1000)),
				scheduler.clone(),
			)
			.subscribe(destination);

		source.unsubscribe();

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}

	#[test]
	fn should_compose() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let composed = compose_operator().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(1000)),
			scheduler,
		);

		let mut subscription = source.clone().pipe(composed).subscribe(destination);

		source.next(1);
		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[SubscriberNotification::Next(1)],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");

		subscription.unsubscribe();
		source.next(3);
		notification_collector.lock().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);
	}
}

mod when_downstream_closes_early {
	use super::*;

	#[test]
	fn should_stop_processing_after_downstream_closes_during_trailing_work() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"throttle_time",
		);

		let mut observable = harness.create_harness_observable().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(1000))
				.with_output(ThrottleOutputBehavior::TrailingOnly),
			scheduler.clone(),
		);
		let destination = harness.create_harness_destination(Some(1));
		harness.register_subscription(observable.subscribe(destination));

		harness.source().next(1);
		harness.source().next(2);

		executor.tick(Duration::from_millis(1000));

		harness.notifications().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		harness.source().next(3);
		executor.tick(Duration::from_millis(0));

		harness.notifications().assert_notifications(
			"throttle_time",
			0,
			[
				SubscriberNotification::Next(2),
				SubscriberNotification::Complete,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(harness.is_subscription_closed());
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"throttle_time",
		);

		let observable = harness.create_harness_observable().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(10))
				.with_output(ThrottleOutputBehavior::TrailingOnly),
			scheduler.clone(),
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error("error");
		harness.assert_terminal_notification(SubscriberNotification::Error("error"));

		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"throttle_time",
		);

		let observable = harness.create_harness_observable().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(10))
				.with_output(ThrottleOutputBehavior::TrailingOnly),
			scheduler.clone(),
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		executor.tick(Duration::from_millis(20));
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"throttle_time",
		);

		let observable = harness.create_harness_observable().throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(10))
				.with_output(ThrottleOutputBehavior::TrailingOnly),
			scheduler.clone(),
		);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		executor.tick(Duration::from_millis(20));
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}
}
