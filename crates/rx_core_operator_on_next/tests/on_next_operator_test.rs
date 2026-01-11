use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_be_able_to_interact_with_the_destination_on_next() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.on_next(move |next, destination| {
			destination.next(next + 10);
			true
		})
		.subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"on_next",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(11),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn should_be_able_to_interact_with_the_destination_on_next_and_prevent_the_original_next() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.on_next(move |next, destination| {
			destination.next(next + 10);
			false
		})
		.subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();

	notification_collector.lock().assert_notifications(
		"on_next",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(11),
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

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.on_next(move |next, destination| {
			destination.next(next + 10);
			true
		})
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("on_next");

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"on_next",
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

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.on_next(move |next, destination| {
			destination.next(next + 10);
			true
		})
		.subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("on_next");

	source.complete();

	notification_collector.lock().assert_notifications(
		"on_next",
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

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().on_next(move |next, destination| {
		destination.next(next + 10);
		true
	});
	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.complete();

	notification_collector.lock().assert_notifications(
		"on_next",
		0,
		[
			SubscriberNotification::Next(11),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("on_next");
		let observable = harness
			.create_harness_observable()
			.on_next(|value, _destination| {
				let _ = value;
				true
			});
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("on_next");
		let observable = harness
			.create_harness_observable()
			.on_next(|value, _destination| {
				let _ = value;
				true
			});
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new("on_next");
		let observable = harness
			.create_harness_observable()
			.on_next(|value, _destination| {
				let _ = value;
				true
			});
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
