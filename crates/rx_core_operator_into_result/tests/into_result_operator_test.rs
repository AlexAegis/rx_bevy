use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_turn_next_emissions_into_results() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().into_result().subscribe(destination);

	source.next(0);
	source.next(1);

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

	let _subscription = source.clone().into_result().subscribe(destination);

	let error = "error";
	source.next(0);
	source.error(error);

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[
			SubscriberNotification::Next(Result::Ok(0)),
			SubscriberNotification::Next(Result::Err(error)),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_turn_complete_normally() {
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let _subscription = source.clone().into_result().subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"into_result",
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
	let destination = MockObserver::<Result<usize, &'static str>>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().into_result();

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"into_result",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
