use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_turn_next_notifications_into_actual_signals() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<ObserverNotification<usize, &'static str>, Never>::default();

	let _subscription = source.clone().dematerialize().subscribe(destination);

	source.next(ObserverNotification::Next(0));
	source.next(ObserverNotification::Next(1));

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_turn_error_notifications_into_actual_error_signals() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<ObserverNotification<usize, &'static str>, Never>::default();

	let _subscription = source.clone().dematerialize().subscribe(destination);

	let error = "error";
	source.next(ObserverNotification::<usize, &'static str>::Error(error));

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

#[test]
fn should_turn_complete_emissions_into_notifications_and_not_complete() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<ObserverNotification<usize, &'static str>, Never>::default();

	let _subscription = source.clone().dematerialize().subscribe(destination);

	source.next(ObserverNotification::Complete);

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_simply_unsubscribe_when_the_source_unsubscribes() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<ObserverNotification<usize, &'static str>, Never>::default();

	let _subscription = source.clone().dematerialize().subscribe(destination);

	source.unsubscribe();

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_be_composable() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<ObserverNotification<usize, &'static str>, Never>::default();

	let composed =
		compose_operator::<ObserverNotification<usize, &'static str>, Never>().dematerialize();

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(ObserverNotification::Next(1));
	source.complete();

	notification_collector.lock().assert_notifications(
		"dematerialize",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);
}
