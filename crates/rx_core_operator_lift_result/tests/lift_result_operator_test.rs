use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_turn_unpack_okay_results_into_nexts() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Result<usize, &'static str>>::default();

	let subscription = source.clone().lift_result().subscribe(destination);

	source.next(Result::Ok(0));
	source.next(Result::Ok(1));
	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_result",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_turn_error_results_into_actual_errors() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Result<usize, &'static str>>::default();

	let subscription = source.clone().lift_result().subscribe(destination);

	let error = "error";
	source.next(Result::Ok(0));
	source.next(Result::Err(error));
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_result",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Error(error),
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Result<usize, &'static str>>::default();

	let subscription = source.clone().lift_result().subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_result",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Result<usize, &'static str>>::default();

	let composed = compose_operator::<Result<usize, &'static str>, Never>().lift_result();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_result",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<
			TestSubject<Result<usize, &'static str>, Never>,
			usize,
			&'static str,
		>::new("lift_result");
		let observable = harness.create_harness_observable().lift_result();
		harness.subscribe_to(observable);
		harness.source().next(Ok(1));
		harness.source().next(Err("error"));
		harness.assert_terminal_notification(SubscriberNotification::Error("error"));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<
			TestSubject<Result<usize, &'static str>, Never>,
			usize,
			&'static str,
		>::new("lift_result");
		let observable = harness.create_harness_observable().lift_result();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<
			TestSubject<Result<usize, &'static str>, Never>,
			usize,
			&'static str,
		>::new("lift_result");
		let observable = harness.create_harness_observable().lift_result();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
