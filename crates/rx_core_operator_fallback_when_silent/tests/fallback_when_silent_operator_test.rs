use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_be_able_to_immediately_next_to_its_destination() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();
	executor.tick(Duration::from_millis(16));
	executor.tick(Duration::from_millis(16)); // To make sure ticks are indexed from subscription

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.fallback_when_silent(|_, _, i| i, scheduler)
		.subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("fallback_when_silent");

	source.next(10);
	// Ticks are indexed since subscription
	executor.tick(Duration::from_millis(16)); // Tick #0: No output, 10 was observed
	executor.tick(Duration::from_millis(16)); // Tick #1
	executor.tick(Duration::from_millis(16)); // Tick #2

	source.next(11);
	executor.tick(Duration::from_millis(16)); // Tick #3: No output, 11 was observed

	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"fallback_when_silent",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(11),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();
}

#[test]
fn should_close_when_errored() {
	let executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.fallback_when_silent(|_, _, _| 10, scheduler)
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("fallback_when_silent");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"fallback_when_silent",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.fallback_when_silent(|_, _, _| 10, scheduler)
		.subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("fallback_when_silent");

	source.complete();

	notification_collector.lock().assert_notifications(
		"fallback_when_silent",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_unsubscribed() {
	let executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.fallback_when_silent(|_, _, _| 10, scheduler)
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("fallback_when_silent");

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"fallback_when_silent",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed =
		compose_operator::<usize, &'static str>().fallback_when_silent(|_, _, _| 10, scheduler);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	executor.tick(Duration::from_millis(16));
	executor.tick(Duration::from_millis(16));
	source.complete();

	notification_collector.lock().assert_notifications(
		"fallback_when_silent",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(10),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"fallback_when_silent",
		);

		let observable = harness
			.create_harness_observable()
			.fallback_when_silent(|_, _, value| value, scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));

		executor.tick(Duration::from_millis(100));
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"fallback_when_silent",
		);

		let observable = harness
			.create_harness_observable()
			.fallback_when_silent(|_, _, value| value, scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		executor.tick(Duration::from_millis(32));
		harness.source().complete();
		executor.tick(Duration::from_millis(100));
		harness.assert_terminal_notification(SubscriberNotification::Complete);

		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"fallback_when_silent",
		);

		let observable = harness
			.create_harness_observable()
			.fallback_when_silent(|_, _, value| value, scheduler.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		executor.tick(Duration::from_millis(32));
		harness.get_subscription_mut().unsubscribe();
		executor.tick(Duration::from_millis(100));
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);

		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}
}
