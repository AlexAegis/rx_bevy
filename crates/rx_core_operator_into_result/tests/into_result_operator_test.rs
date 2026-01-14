use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_turn_next_emissions_into_results() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().into_result().subscribe(destination);

	source.next(0);
	source.next(1);
	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[
			SubscriberNotification::Next(Result::Ok(0)),
			SubscriberNotification::Next(Result::Ok(1)),
		],
		true,
	);
}

#[test]
fn should_turn_error_emissions_into_results_and_not_error() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().into_result().subscribe(destination);

	let error = "error";
	source.next(0);
	source.error(error);
	assert!(
		!subscription.is_closed(),
		"Should not close as the error turned into a next!"
	);

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[
			SubscriberNotification::Next(Result::Ok(0)),
			SubscriberNotification::Next(Result::Err(error)),
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().into_result().subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().into_result();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

/// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, Result<usize, TestError>, Never>::new(
				"into_result",
			);
		let observable = harness.create_harness_observable().into_result();
		harness.subscribe_to(observable);
		harness.source().next(1);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<usize, TestError>, Result<usize, TestError>, Never>::new(
				"into_result",
			);
		let observable = harness.create_harness_observable().into_result();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
