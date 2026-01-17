use rx_core::prelude::*;
use rx_core_common::{ComposeOperator, SubscriberNotification};
use rx_core_testing::prelude::*;

type MapComposable = ComposeOperator<MapOperator<usize, TestError, fn(usize) -> usize, usize>>;

fn add_10(v: usize) -> usize {
	v + 10
}

#[test]
fn should_apply_inner_operator_when_some() {
	let destination = MockObserver::<usize, TestError>::default();
	let notifications = destination.get_notification_collector();
	let mut source = PublishSubject::<usize, TestError>::default();

	let composable = Some(ComposeOperator::from(MapOperator::new(
		add_10 as fn(usize) -> usize,
	)));
	let mut subscription = source.clone().pipe(composable).subscribe(destination);
	let teardown = subscription.add_tracked_teardown("option_operator_some");

	source.next(1);
	source.next(2);
	source.complete();

	notifications.lock().assert_notifications(
		"option_operator_some",
		0,
		[
			SubscriberNotification::Next(11),
			SubscriberNotification::Next(12),
			SubscriberNotification::Complete,
		],
		true,
	);

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

#[test]
fn should_passthrough_when_none() {
	let destination = MockObserver::<usize, TestError>::default();
	let notifications = destination.get_notification_collector();
	let mut source = PublishSubject::<usize, TestError>::default();

	let composable: Option<MapComposable> = None;
	let mut subscription = source.clone().pipe(composable).subscribe(destination);
	let teardown = subscription.add_tracked_teardown("option_operator_none");

	source.next(5);
	source.complete();

	notifications.lock().assert_notifications(
		"option_operator_none",
		0,
		[
			SubscriberNotification::Next(5),
			SubscriberNotification::Complete,
		],
		true,
	);

	teardown.assert_was_torn_down();
	assert!(subscription.is_closed());
}

mod contracts_none {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_some",
		);
		let composable: Option<MapComposable> = None;
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.source().next(1);
		harness.source().error(TestError);

		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_some",
		);
		let composable: Option<MapComposable> = None;
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.source().complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_some",
		);
		let composable: Option<MapComposable> = None;
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}

mod contracts_some {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_none",
		);
		let composable = Some(ComposeOperator::from(MapOperator::new(
			add_10 as fn(usize) -> usize,
		)));
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.source().next(1);
		harness.source().error(TestError);

		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_none",
		);
		let composable = Some(ComposeOperator::from(MapOperator::new(
			add_10 as fn(usize) -> usize,
		)));
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.source().complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<TestSubject<usize, TestError>, usize, TestError>::new(
			"option_operator_none",
		);
		let composable = Some(ComposeOperator::from(MapOperator::new(
			add_10 as fn(usize) -> usize,
		)));
		let observable = harness.create_harness_observable().pipe(composable);
		harness.subscribe_to(observable);

		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
