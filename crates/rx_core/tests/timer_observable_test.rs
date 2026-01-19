use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_common::{
	Observable, SubscriberNotification, SubscriptionLike, TeardownCollectionExtension,
};
use rx_core_testing::prelude::*;

#[test]
fn should_send_a_unit_value_after_the_specified_amount_of_time() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut timer = timer(Duration::from_millis(200), scheduler);

	let _subscription = timer.subscribe(destination);
	executor.tick(Duration::from_millis(199));

	assert!(
		notification_collector.lock().is_empty(),
		"there should be no notifications yet"
	);

	executor.tick(Duration::from_millis(1));

	notification_collector.lock().assert_notifications(
		"timer",
		0,
		[
			SubscriberNotification::Next(()),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_not_send_a_unit_value_after_the_specified_amount_of_time_when_unsubscribed() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut timer = TimerObservable::new(Duration::from_millis(200), scheduler);

	let mut subscription = timer.subscribe(destination);
	executor.tick(Duration::from_millis(199));

	assert!(
		notification_collector.lock().is_empty(),
		"there should be no notifications yet"
	);

	subscription.unsubscribe();

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Unsubscribe,
		"unsubscribe notification not received"
	);

	executor.tick(Duration::from_millis(10000));

	assert!(
		!notification_collector.lock().nth_notification_exists(1),
		"an extra notification was observed"
	);
}

#[test]
fn should_execute_teardowns_when_unsubscribed() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();

	let mut timer = TimerObservable::new(Duration::from_millis(200), scheduler);

	let mut subscription = timer.subscribe(destination);

	let was_teardown_executed = Arc::new(AtomicBool::new(false));
	let was_teardown_executed_clone = was_teardown_executed.clone();
	subscription.add_fn(move || {
		was_teardown_executed_clone.store(true, Ordering::Relaxed);
	});

	executor.tick(Duration::from_millis(199));

	subscription.unsubscribe();

	assert!(
		was_teardown_executed.load(Ordering::Relaxed),
		"teardown was not executed"
	);

	assert!(subscription.is_closed(), "the subscription was not closed");
}

/// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<_, (), Never>::new_with_source(
			"timer",
			timer(Duration::from_millis(10), scheduler.clone()),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		executor.tick(Duration::from_millis(20));
		harness.assert_terminal_notification(SubscriberNotification::Complete);
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness = TestHarness::<_, (), Never>::new_with_source(
			"timer",
			timer(Duration::from_millis(10), scheduler.clone()),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		executor.tick(Duration::from_millis(20));
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
		assert!(executor.is_empty(), "rx_verify_scheduler_is_empty");
	}
}
