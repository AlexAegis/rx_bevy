use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_be_able_to_immediately_next_to_its_destination() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().start_with(10).subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"start_with",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().start_with(10).subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("start_with");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"start_with",
		0,
		[
			SubscriberNotification::Next(10),
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
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().start_with(10).subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("start_with");

	source.complete();

	notification_collector.lock().assert_notifications(
		"start_with",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().start_with(10);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"start_with",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}
