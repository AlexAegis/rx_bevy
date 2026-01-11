use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_forward_all_notifications_to_the_tap_destination_too() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_destination = MockObserver::<usize, &'static str>::default();
	let tap_notification_collector = tap_destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().tap(tap_destination).subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"tap",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);

	tap_notification_collector.lock().assert_notifications(
		"tap_destination",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_destination = MockObserver::<usize, &'static str>::default();
	let tap_notification_collector = tap_destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().tap(tap_destination).subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("tap");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"tap",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	tap_notification_collector.lock().assert_notifications(
		"tap_destination",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_destination = MockObserver::<usize, &'static str>::default();
	let tap_notification_collector = tap_destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().tap(tap_destination).subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("tap");

	source.complete();

	notification_collector.lock().assert_notifications(
		"tap",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	tap_notification_collector.lock().assert_notifications(
		"tap_destination",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let tap_destination = MockObserver::<usize, &'static str>::default();
	let tap_notification_collector = tap_destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().tap(tap_destination);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"tap",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	tap_notification_collector.lock().assert_notifications(
		"tap_destination",
		0,
		[SubscriberNotification::Complete],
		true,
	);

	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("tap");
		let observable = harness
			.create_harness_observable()
			.tap(MockObserver::default());
		harness.subscribe_to(observable);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("tap");
		let observable = harness
			.create_harness_observable()
			.tap(MockObserver::default());
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("tap");
		let observable = harness
			.create_harness_observable()
			.tap(MockObserver::default());
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
