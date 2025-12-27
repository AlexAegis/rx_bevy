use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_be_a_noop_operator() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let _subscription = source.clone().pipe(composable).subscribe(destination);

	source.next(0);
	source.next(1);

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_just_forward_complete_calls() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let _subscription = source.clone().pipe(composable).subscribe(destination);

	source.next(0);
	source.next(1);
	source.complete();

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_just_forward_error_calls() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composable = compose_operator::<usize, &'static str>();
	let _subscription = source.clone().pipe(composable).subscribe(destination);

	let error = "error";
	source.next(0);
	source.next(1);
	source.error(error);

	notification_collector.lock().assert_notifications(
		"identity",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
