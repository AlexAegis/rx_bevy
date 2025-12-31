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
fn should_retry_on_later_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let error = "error";
	let mut retried = source
		.clone()
		.on_next(|next, destination| {
			if *next > 10 {
				destination.error(error);
				false
			} else {
				true
			}
		})
		.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	source.next(1);
	source.next(99); // First retry!
	source.next(2);
	source.next(99); // Second retry!
	source.next(3);
	source.next(99); // Error will go through!
	source.next(4);

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

// #[test] // TODO: Fix, locks up
fn _should_retry_on_mixed_immediate_and_later_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let error = "error";
	let mut retried = concat((
		(0..=1)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		source.clone().on_next(|next, destination| {
			if *next > 10 {
				destination.error(error);
				false
			} else {
				true
			}
		}),
	))
	.retry(2);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	source.next(2);
	println!("omg");
	source.next(99); // First retry!
	source.next(3);
	source.next(99); // Second retry!
	source.next(4);
	source.next(99); // Error will go through!
	source.next(5);

	notification_collector.print();

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(4),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());

	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<Never, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let mut retried = throw(error).retry(100);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
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
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut retried = (0..=2).into_observable().retry(1);

	let mut subscription = retried.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("retry - destination");

	notification_collector.lock().assert_notifications(
		"retry - destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}
