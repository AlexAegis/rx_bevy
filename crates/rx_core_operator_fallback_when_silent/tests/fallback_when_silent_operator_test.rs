use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

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
			SubscriberNotification::Unsubscribe,
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
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
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
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
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
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}
