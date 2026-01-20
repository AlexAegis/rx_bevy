use std::time::Duration;

use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_emit_value_at_index_and_complete() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().element_at(2).subscribe(destination);

	source.next(10);
	source.next(20);
	source.next(30);
	assert!(subscription.is_closed());
	source.next(40);

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[
			SubscriberNotification::Next(30),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_be_composable() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().element_at(1);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(5);
	source.next(6);
	assert!(subscription.is_closed());
	source.next(7);

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_forward_upstream_errors_wrapped() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().element_at(0).subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[SubscriberNotification::Error(
			ElementAtOperatorError::Upstream(error),
		)],
		true,
	);
}

#[test]
fn should_error_when_out_of_range_without_default() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().element_at(3).subscribe(destination);

	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[SubscriberNotification::Error(
			ElementAtOperatorError::IndexOutOfRange {
				requested_index: 3,
				observed_nexts: 2,
			},
		)],
		true,
	);
}

#[test]
fn should_emit_default_when_out_of_range_with_default() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.element_at_or_else(1, ProvideWithCloneOf(99))
		.subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[
			SubscriberNotification::Next(99),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_emit_default_when_out_of_range_with_default_on_compose() {
	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed =
		compose_operator::<usize, &'static str>().element_at_or_else(1, ProvideWithCloneOf(42));

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[
			SubscriberNotification::Next(42),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_ignore_values_after_complete_even_with_delay_downstream() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let destination = MockObserver::<usize, ElementAtOperatorError<&'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.element_at(0)
		.delay(Duration::from_millis(10), scheduler.clone())
		.subscribe(destination);

	source.next(1);
	source.complete();
	source.next(2);

	executor.tick(Duration::from_millis(10));

	notification_collector.lock().assert_notifications(
		"element_at",
		0,
		[
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
		let mut harness = TestHarness::<
			TestSubject<usize, MockError>,
			usize,
			ElementAtOperatorError<MockError>,
		>::new("element_at");
		let observable = harness.create_harness_observable().element_at(0);
		harness.subscribe_to(observable);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(
			ElementAtOperatorError::Upstream(MockError),
		));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<
			TestSubject<usize, MockError>,
			usize,
			ElementAtOperatorError<MockError>,
		>::new("element_at");
		let observable = harness.create_harness_observable().element_at(0);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<
			TestSubject<usize, MockError>,
			usize,
			ElementAtOperatorError<MockError>,
		>::new("element_at");
		let observable = harness.create_harness_observable().element_at(0);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
