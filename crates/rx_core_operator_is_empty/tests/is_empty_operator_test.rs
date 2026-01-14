use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_immediately_emit_false_if_upstream_emits_something() {
	let destination = MockObserver::<bool, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().is_empty().subscribe(destination);

	source.next(0);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"is_empty",
		0,
		[
			SubscriberNotification::Next(false),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_emit_true_if_upstream_does_not_emit_before_it_would_complete() {
	let destination = MockObserver::<bool, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().is_empty().subscribe(destination);

	source.complete();

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"is_empty",
		0,
		[
			SubscriberNotification::Next(true),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<bool, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().is_empty();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"is_empty",
		0,
		[
			SubscriberNotification::Next(true),
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
			TestHarness::<TestSubject<usize, TestError>, bool, TestError>::new("is_empty");
		let observable = harness.create_harness_observable().is_empty();
		harness.subscribe_to(observable);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, bool, TestError>::new("is_empty");
		let observable = harness.create_harness_observable().is_empty();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, bool, TestError>::new("is_empty");
		let observable = harness.create_harness_observable().is_empty();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
