use rx_core::prelude::*;
use rx_core_common::Observable;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_the_result_once_completed() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.reduce(|acc, next| acc + next, 0)
		.subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"reduce",
		0,
		[
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_emit_the_seed_once_completed_without_any_nexts() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.reduce(|acc, next| acc + next, 0)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"reduce",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Complete,
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.reduce(|acc, next| acc + next, 0)
		.subscribe(destination);

	source.next(0);
	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"reduce",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_unsubscribe_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.reduce(|acc, next| acc + next, 0)
		.subscribe(destination);

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"reduce",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator().reduce(|acc, next| acc + next, 0);

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"reduce",
		0,
		[
			SubscriberNotification::Next(6),
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
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("reduce");
		let observable = harness
			.create_harness_observable()
			.reduce(|acc, next| acc + next, 0);
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().error(MockError);
		harness.assert_terminal_notification(SubscriberNotification::Error(MockError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("reduce");
		let observable = harness
			.create_harness_observable()
			.reduce(|acc, next| acc + next, 0);
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, MockError>, usize, MockError>::new("reduce");
		let observable = harness
			.create_harness_observable()
			.reduce(|acc, next| acc + next, 0);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
