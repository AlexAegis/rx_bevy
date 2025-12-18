use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};

use rx_core::prelude::*;
use rx_core_testing::{MockExecutor, prelude::*};
use rx_core_traits::{
	Observable, SubscriberNotification, SubscriptionLike, TeardownCollectionExtension,
};

use crate::observable::TimerObservable;

#[test]
pub fn should_send_a_unit_value_after_the_specified_amount_of_time() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut timer = TimerObservable::new(Duration::from_millis(200), scheduler);

	let _subscription = timer.subscribe(destination);
	executor.tick(Duration::from_millis(199));

	assert!(
		notification_collector.lock().is_empty(),
		"there should be no notifications yet"
	);

	executor.tick(Duration::from_millis(1));

	assert_eq!(
		notification_collector.lock().nth_notification(0),
		&SubscriberNotification::Next(()),
		"next notification not received"
	);

	assert_eq!(
		notification_collector.lock().nth_notification(1),
		&SubscriberNotification::Complete,
		"complete notification not received"
	);

	assert_eq!(
		notification_collector.lock().nth_notification(2),
		&SubscriberNotification::Unsubscribe,
		"unsubscribe notification not received"
	);

	assert!(
		!notification_collector.lock().nth_notification_exists(3),
		"an extra notification was observed"
	);
}

#[test]
pub fn should_not_send_a_unit_value_after_the_specified_amount_of_time_when_unsubscribed() {
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
pub fn should_execute_teardowns_when_unsubscribed() {
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
