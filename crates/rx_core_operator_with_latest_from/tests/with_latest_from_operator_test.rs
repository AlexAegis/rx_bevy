use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_emit_values_with_latest_from() {
	let destination = MockObserver::<(usize, &'static str), &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let _subscription = source
		.clone()
		.with_latest_from(inner.clone())
		.subscribe(destination);

	source.next(0);
	inner.next("foo");
	source.next(1);
	source.next(2);
	inner.next("bar");
	source.next(3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"with_latest_from",
		0,
		[
			SubscriberNotification::Next((1, "foo")),
			SubscriberNotification::Next((2, "foo")),
			SubscriberNotification::Next((3, "bar")),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_error_when_inner_observable_errors() {
	let destination = MockObserver::<(usize, &'static str), &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();
	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let mut subscription = source
		.clone()
		.with_latest_from(inner.clone())
		.subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("with_latest_from");

	let error = "error";
	inner.error(error);
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"with_latest_from",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_not_complete_when_inner_observable_completes_but_primed() {
	let destination = MockObserver::<(usize, &'static str), &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let mut inner_teardown = SharedSubscription::default();
	let tracked_inner_teardown = inner_teardown
		.clone()
		.add_tracked_teardown("inner_observable");

	let mut subscription = source
		.clone()
		.with_latest_from(inner.clone().finalize(move || inner_teardown.unsubscribe()))
		.subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("with_latest_from");

	inner.next("foo");
	inner.complete();
	source.next(1);
	assert!(!subscription.is_closed());
	tracked_teardown.assert_yet_to_be_torn_down();
	tracked_inner_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"with_latest_from",
		0,
		[SubscriberNotification::Next((1, "foo"))],
		true,
	);
}

#[test]
fn should_complete_when_inner_observable_completes_but_not_primed() {
	let destination = MockObserver::<(usize, &'static str), &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let mut inner_teardown = SharedSubscription::default();
	let tracked_inner_teardown = inner_teardown
		.clone()
		.add_tracked_teardown("inner_observable");

	let mut subscription = source
		.clone()
		.with_latest_from(inner.clone().finalize(move || inner_teardown.unsubscribe()))
		.subscribe(destination);
	let tracked_teardown = subscription.add_tracked_teardown("with_latest_from");

	inner.complete();
	source.next(1);
	assert!(subscription.is_closed());
	tracked_teardown.assert_was_torn_down();
	tracked_inner_teardown.assert_was_torn_down();

	notification_collector.lock().assert_notifications(
		"with_latest_from",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<(usize, &'static str), &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let mut inner = PublishSubject::<&'static str, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().with_latest_from(inner.clone());

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	inner.next("foo");
	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"with_latest_from",
		0,
		[
			SubscriberNotification::Next((1, "foo")),
			SubscriberNotification::Next((2, "foo")),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let inner = PublishSubject::<&'static str, TestError>::default();

		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, (usize, &'static str), TestError>::new(
				"with_latest_from",
			);
		let observable = harness
			.create_harness_observable()
			.with_latest_from(inner.clone());
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let inner = PublishSubject::<&'static str, TestError>::default();

		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, (usize, &'static str), TestError>::new(
				"with_latest_from",
			);
		let observable = harness
			.create_harness_observable()
			.with_latest_from(inner.clone());
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let inner = PublishSubject::<&'static str, TestError>::default();

		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, (usize, &'static str), TestError>::new(
				"with_latest_from",
			);
		let observable = harness
			.create_harness_observable()
			.with_latest_from(inner.clone());
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
