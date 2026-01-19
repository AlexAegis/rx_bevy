use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_emit_values_pairwise() {
	let destination = MockObserver::<[usize; 2], &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().pairwise().subscribe(destination);

	for i in 0..=6 {
		source.next(i);
	}

	assert!(!subscription.is_closed());

	source.complete();

	notification_collector.lock().assert_notifications(
		"pairwise",
		0,
		[
			SubscriberNotification::Next([0, 1]),
			SubscriberNotification::Next([1, 2]),
			SubscriberNotification::Next([2, 3]),
			SubscriberNotification::Next([3, 4]),
			SubscriberNotification::Next([4, 5]),
			SubscriberNotification::Next([5, 6]),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<[usize; 2], &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().pairwise().subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("pairwise");

	source.next(1);
	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"pairwise",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<[usize; 2], &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().pairwise().subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("pairwise");

	source.complete();
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"pairwise",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_unsubscribe_normally() {
	let destination = MockObserver::<[usize; 2], &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source.clone().pairwise().subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("pairwise");

	subscription.unsubscribe();

	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"pairwise",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<[usize; 2], &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().pairwise();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"pairwise",
		0,
		[
			SubscriberNotification::Next([0, 1]),
			SubscriberNotification::Next([1, 2]),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, [usize; 2], MockError>::new("pairwise");
		let observable = harness.create_harness_observable().pairwise();
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, [usize; 2], MockError>::new("pairwise");
		let observable = harness.create_harness_observable().pairwise();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, [usize; 2], MockError>::new("pairwise");
		let observable = harness.create_harness_observable().pairwise();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
