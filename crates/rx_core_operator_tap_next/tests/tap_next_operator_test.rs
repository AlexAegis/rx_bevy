use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_forward_next_notifications_to_the_tap_fn_too() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
	let tap_notification_collector_clone = tap_notification_collector.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.tap_next(move |next| {
			tap_notification_collector_clone
				.lock()
				.push(SubscriberNotification::Next(*next))
		})
		.subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"tap_next",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	tap_notification_collector.lock().assert_notifications(
		"tap_destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
	let tap_notification_collector_clone = tap_notification_collector.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.tap_next(move |next| {
			tap_notification_collector_clone
				.lock()
				.push(SubscriberNotification::Next(*next))
		})
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("tap_next");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"tap_next",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	tap_notification_collector
		.lock()
		.assert_is_empty("tap_destination");

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
	let tap_notification_collector_clone = tap_notification_collector.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.tap_next(move |next| {
			tap_notification_collector_clone
				.lock()
				.push(SubscriberNotification::Next(*next))
		})
		.subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("tap_next");

	source.complete();

	notification_collector.lock().assert_notifications(
		"tap_next",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	tap_notification_collector
		.lock()
		.assert_is_empty("tap_destination");

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_notification_collector = NotificationCollector::<usize, &'static str>::default();
	let tap_notification_collector_clone = tap_notification_collector.clone();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().tap_next(move |next| {
		tap_notification_collector_clone
			.lock()
			.push(SubscriberNotification::Next(*next))
	});

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"tap_next",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	tap_notification_collector
		.lock()
		.assert_is_empty("tap_destination");

	assert!(subscription.is_closed());
}
