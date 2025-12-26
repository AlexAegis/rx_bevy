use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::{MockExecutor, prelude::*};

#[test]
fn should_emit_from_all_sources_when_any_of_them_nexts_with_default_options() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(IntervalObservableOptions::default(), scheduler);

	let _s = interval_observable.subscribe(destination);

	notification_collector
		.lock()
		.assert_is_empty("interval - before any ticks");

	executor.tick(Duration::from_millis(500));

	notification_collector
		.lock()
		.assert_is_empty("interval - after a small tick");

	executor.tick(Duration::from_millis(500));

	notification_collector.lock().assert_notifications(
		"interval - first emission",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);

	executor.tick(Duration::from_millis(2000));

	notification_collector.lock().assert_notifications(
		"interval - second emission",
		1,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);
}

#[test]
fn should_only_emit_at_most_max_emissions_per_tick_for_large_ticks() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(100),
			start_on_subscribe: false,
			max_emissions_per_tick: 2,
		},
		scheduler,
	);

	let _s = interval_observable.subscribe(destination);

	notification_collector
		.lock()
		.assert_is_empty("interval - before any ticks");

	executor.tick(Duration::from_millis(5000));

	notification_collector.lock().assert_notifications(
		"interval - first emission",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	executor.tick(Duration::from_millis(2000));

	notification_collector.lock().assert_notifications(
		"interval - second emission",
		2,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
		],
		true,
	);
}

#[test]
fn should_only_emit_at_most_once_if_max_emission_per_tick_is_zero() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(100),
			start_on_subscribe: false,
			max_emissions_per_tick: 0,
		},
		scheduler,
	);

	let _s = interval_observable.subscribe(destination);

	notification_collector
		.lock()
		.assert_is_empty("interval - before any ticks");

	executor.tick(Duration::from_millis(5000));

	notification_collector.lock().assert_notifications(
		"interval - first emission",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);

	executor.tick(Duration::from_millis(2000));

	notification_collector.lock().assert_notifications(
		"interval - second emission",
		1,
		[SubscriberNotification::Next(1)],
		true,
	);
}

#[test]
fn should_emit_immediately_when_start_on_subscribe_is_true() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(100),
			start_on_subscribe: true,
			max_emissions_per_tick: 20,
		},
		scheduler,
	);

	let _s = interval_observable.subscribe(destination);

	notification_collector.lock().assert_notifications(
		"interval - immediate emission",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);

	executor.tick(Duration::from_millis(200));

	notification_collector.lock().assert_notifications(
		"interval - scheduled emission",
		1,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);
}

#[test]
fn should_stop_emitting_when_unsubscribed() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(1000),
			start_on_subscribe: false,
			max_emissions_per_tick: 10,
		},
		scheduler,
	);

	let mut subscription = interval_observable.subscribe(destination);

	executor.tick(Duration::from_millis(1000));

	notification_collector.lock().assert_notifications(
		"interval - first emission",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);

	executor.tick(Duration::from_millis(2000));

	notification_collector.lock().assert_notifications(
		"interval - second emission",
		1,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
		],
		true,
	);

	assert!(!executor.is_empty());

	subscription.unsubscribe();

	executor.tick(Duration::from_millis(2000));

	notification_collector.lock().assert_notifications(
		"interval - after unsubscribed",
		3,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	assert!(executor.is_empty());
}

#[test]
fn should_stop_emitting_when_downstream_is_closed() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut interval_observable = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(1000),
			start_on_subscribe: false,
			max_emissions_per_tick: 10,
		},
		scheduler,
	)
	.take(2);

	let subscription = interval_observable.subscribe(destination);

	executor.tick(Duration::from_millis(1000));

	notification_collector.lock().assert_notifications(
		"interval - first emission",
		0,
		[SubscriberNotification::Next(0)],
		true,
	);

	assert!(!executor.is_empty());
	executor.tick(Duration::from_millis(10000));
	assert!(executor.is_empty());

	notification_collector.lock().assert_notifications(
		"interval - second emission",
		1,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete, // Came from `take`, not `interval`!
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}
