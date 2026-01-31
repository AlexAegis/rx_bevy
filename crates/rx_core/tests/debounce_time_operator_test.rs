use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::Observable;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_only_the_latest_value_after_silence() {
	println!("cwd: {:?}", std::env::current_dir().unwrap());

	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.debounce_time(Duration::from_millis(1000), scheduler.clone())
		.subscribe(destination);

	source.next(1);
	executor.tick(Duration::from_millis(500));
	source.next(2);

	notification_collector
		.lock()
		.assert_is_empty("debounce_time");

	executor.tick(Duration::from_millis(999));
	notification_collector
		.lock()
		.assert_is_empty("debounce_time");

	executor.tick(Duration::from_millis(1));
	notification_collector.lock().assert_notifications(
		"debounce_time",
		0,
		[SubscriberNotification::Next(2)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

mod error {
	use super::*;

	#[test]
	fn should_error_immediately_and_cancel_pending_debounced_values() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.debounce_time(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		source.next(1);
		let error = "error";
		source.error(error);

		notification_collector.lock().assert_notifications(
			"debounce_time",
			0,
			[SubscriberNotification::Error(error)],
			true,
		);

		executor.tick(Duration::from_millis(0));
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
		assert!(subscription.is_closed());
	}
}

mod complete {
	use super::*;

	#[test]
	fn should_complete_after_debounced_emission_when_pending() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.debounce_time(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		source.next(1);
		source.complete();

		notification_collector
			.lock()
			.assert_is_empty("debounce_time");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"debounce_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
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
			.debounce_time(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		source.complete();

		notification_collector.lock().assert_notifications(
			"debounce_time",
			0,
			[SubscriberNotification::Complete],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}
}

mod unsubscribe {
	use super::*;

	#[test]
	fn should_unsubscribe_after_debounced_emission_when_pending() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();

		let destination = MockObserver::<usize, &'static str>::default();
		let notification_collector = destination.get_notification_collector();

		let mut source = PublishSubject::<usize, &'static str>::default();
		let subscription = source
			.clone()
			.debounce_time(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		source.next(1);
		source.unsubscribe();

		notification_collector
			.lock()
			.assert_is_empty("debounce_time");

		executor.tick(Duration::from_millis(1000));

		notification_collector.lock().assert_notifications(
			"debounce_time",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

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
			.debounce_time(Duration::from_millis(1000), scheduler.clone())
			.subscribe(destination);

		source.unsubscribe();

		notification_collector.lock().assert_notifications(
			"debounce_time",
			0,
			[SubscriberNotification::Unsubscribe],
			true,
		);

		assert!(executor.is_empty(), "All work should be finished by now");
		assert!(subscription.is_closed());
	}
}

#[test]
fn should_compose() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let composed = compose_operator().debounce_time(Duration::from_millis(1000), scheduler);

	let mut subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	executor.tick(Duration::from_millis(1000));

	notification_collector.lock().assert_notifications(
		"debounce_time",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(executor.is_empty(), "All work should be finished by now");

	subscription.unsubscribe();
}

mod contracts {
	use super::*;

	/// Errors are instant, so no tick is required to observe the error signal.
	#[test]
	fn rx_contract_closed_after_error() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, &'static str>, usize, &'static str>::new(
			"debounce_time",
		);

		let observable = harness
			.create_harness_observable()
			.debounce_time(Duration::from_millis(10), scheduler.clone());
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
			"debounce_time",
		);

		let observable = harness
			.create_harness_observable()
			.debounce_time(Duration::from_millis(10), scheduler.clone());
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
			"debounce_time",
		);

		let observable = harness
			.create_harness_observable()
			.debounce_time(Duration::from_millis(10), scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.get_subscription_mut().unsubscribe();
		executor.tick(Duration::from_millis(20));
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}
}
