use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_upstream_value_with_an_index_attached_starting_from_zero() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().enumerate().subscribe(destination);

	source.next(10);
	source.next(20);
	source.next(30);

	notification_collector.lock().assert_notifications(
		"enumerate",
		0,
		[
			SubscriberNotification::Next((10, 0)),
			SubscriberNotification::Next((20, 1)),
			SubscriberNotification::Next((30, 2)),
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().enumerate().subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"enumerate",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().enumerate().subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"enumerate",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().enumerate();

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(10);
	source.next(20);
	source.next(30);

	notification_collector.lock().assert_notifications(
		"enumerate",
		0,
		[
			SubscriberNotification::Next((10, 0)),
			SubscriberNotification::Next((20, 1)),
			SubscriberNotification::Next((30, 2)),
		],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, (usize, usize), MockError>::new(
				"enumerate",
			);
		let observable = harness.create_harness_observable().enumerate();
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, (usize, usize), MockError>::new(
				"enumerate",
			);
		let observable = harness.create_harness_observable().enumerate();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, (usize, usize), MockError>::new(
				"enumerate",
			);
		let observable = harness.create_harness_observable().enumerate();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
