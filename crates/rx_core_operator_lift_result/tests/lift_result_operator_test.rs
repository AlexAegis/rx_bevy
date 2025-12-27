use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

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
			SubscriberNotification::Unsubscribe,
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
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
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
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
