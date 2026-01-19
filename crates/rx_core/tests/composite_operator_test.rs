use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_create_a_new_operator_from_two() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composite = CompositeOperator::new(
		MapOperator::new(|i| i * 2),
		MapOperator::new(|i| format!("{i}")),
	);

	let subscription = source.clone().pipe(composite).subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"composite",
		0,
		[
			SubscriberNotification::Next("0".to_string()),
			SubscriberNotification::Next("2".to_string()),
		],
		true,
	);
}

#[test]
fn should_create_a_new_operator_from_two_using_compose_with() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composite = MapOperator::new(|i| i * 2).compose_with(MapOperator::new(|i| format!("{i}")));

	let subscription = source.clone().pipe(composite).subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"composite",
		0,
		[
			SubscriberNotification::Next("0".to_string()),
			SubscriberNotification::Next("2".to_string()),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_create_a_new_operator_from_two_and_error() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composite = CompositeOperator::new(
		MapOperator::new(|i| i * 2),
		MapOperator::new(|i| format!("{i}")),
	);

	let subscription = source.clone().pipe(composite).subscribe(destination);
	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"composite",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_create_a_new_operator_from_two_and_complete() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composite = CompositeOperator::new(
		MapOperator::new(|i| i * 2),
		MapOperator::new(|i| format!("{i}")),
	);

	let subscription = source.clone().pipe(composite).subscribe(destination);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"composite",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, String, MockError>::new("composite");
		let observable = harness
			.create_harness_observable()
			.pipe(MapOperator::new(|i| i * 2).compose_with(MapOperator::new(|i| format!("{i}"))));
		harness.subscribe_to(observable);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, String, MockError>::new("composite");
		let observable = harness
			.create_harness_observable()
			.pipe(MapOperator::new(|i| i * 2).compose_with(MapOperator::new(|i| format!("{i}"))));
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, String, MockError>::new("composite");
		let observable = harness
			.create_harness_observable()
			.pipe(MapOperator::new(|i| i * 2).compose_with(MapOperator::new(|i| format!("{i}"))));
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
