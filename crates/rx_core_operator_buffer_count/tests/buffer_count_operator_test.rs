use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_buffer_values_up_to_buffer_size() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().buffer_count(3).subscribe(destination);

	for i in 0..=10 {
		source.next(i);
	}

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[
			SubscriberNotification::Next(vec![0, 1, 2]),
			SubscriberNotification::Next(vec![3, 4, 5]),
			SubscriberNotification::Next(vec![6, 7, 8]),
		],
		true,
	);
}

#[test]
fn should_error_normally_without_emitting_anything_even_if_there_was_a_buffer() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().buffer_count(3).subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("buffer_count");

	source.next(1);
	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally_without_emitting_anything_if_there_is_no_buffer_yet() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().buffer_count(2).subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("buffer_count");

	source.complete();
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_complete_normally_without_emitting_anything_if_there_is_no_buffer_anymore() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().buffer_count(2).subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("buffer_count");

	source.next(1);
	source.next(2);
	source.next(3);
	source.next(4);
	source.complete();
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[
			SubscriberNotification::Next(vec![1, 2]),
			SubscriberNotification::Next(vec![3, 4]),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_unsubscribe_normally() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().buffer_count(2).subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("buffer_count");

	subscription.unsubscribe();

	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<Vec<usize>, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().buffer_count(2);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"buffer_count",
		0,
		[
			SubscriberNotification::Next(vec![0, 1]),
			SubscriberNotification::Next(vec![2]),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<TestSubject<usize, MockError>, Vec<usize>, MockError>::new(
			"buffer_count",
		);
		let observable = harness.create_harness_observable().buffer_count(2);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<TestSubject<usize, MockError>, Vec<usize>, MockError>::new(
			"buffer_count",
		);
		let observable = harness.create_harness_observable().buffer_count(2);
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<TestSubject<usize, MockError>, Vec<usize>, MockError>::new(
			"buffer_count",
		);
		let observable = harness.create_harness_observable().buffer_count(2);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
