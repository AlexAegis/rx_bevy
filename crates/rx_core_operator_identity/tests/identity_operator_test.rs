use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_be_a_noop_operator() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let subscription = source.clone().pipe(composable).subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_just_forward_complete_calls() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let subscription = source.clone().pipe(composable).subscribe(destination);

	source.next(0);
	source.next(1);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_just_forward_error_calls() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let mut subscription = source.clone().pipe(composable).subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("identity");

	let error = "error";
	source.next(0);
	source.next(1);
	source.error(error);
	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
		],
		true,
	);
}
