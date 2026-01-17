use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_filter_next_emissions_using_the_predicate_provided() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter(|i, _| i % 2 == 0)
		.subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.next(4);

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter",
		0,
		[
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(4),
		],
		true,
	);
}

#[test]
fn should_filter_based_on_index() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter(|_value, index| index % 2 == 0)
		.subscribe(destination);

	source.next(99);
	source.next(98);
	source.next(97);
	source.next(96);

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter",
		0,
		[
			SubscriberNotification::Next(99),
			SubscriberNotification::Next(97),
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter(|i, _| i % 2 == 0)
		.subscribe(destination);

	let error = "error";
	source.error(error);

	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.filter(|i, _| i % 2 == 0)
		.subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().filter(|i, _| i % 2 == 0);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.next(2);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"filter",
		0,
		[
			SubscriberNotification::Next(2),
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
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("filter");
		let observable = harness
			.create_harness_observable()
			.filter(|value, _| value % 2 == 0);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("filter");
		let observable = harness
			.create_harness_observable()
			.filter(|value, _| value % 2 == 0);
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("filter");
		let observable = harness
			.create_harness_observable()
			.filter(|value, _| value % 2 == 0);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
