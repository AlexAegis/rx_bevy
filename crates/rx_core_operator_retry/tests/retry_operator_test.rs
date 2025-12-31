use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_retry_on_immediate_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let mut retried = concat((
		(0..=2)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		throw(error).map(Never::map_into::<usize>()),
	))
	.retry(1);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_retry_on_later_errors() {}

#[test]
fn should_retry_on_mixed_immediate_and_later_errors() {}

#[test]
fn should_close_when_errored() {}

#[test]
fn should_close_when_completed() {}
