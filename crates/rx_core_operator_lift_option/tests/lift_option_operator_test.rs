use rx_core::prelude::*;
use rx_core_common::{Observable, SubscriberNotification};
use rx_core_testing::prelude::*;

#[test]
fn should_turn_unpack_okay_results_into_nexts() {
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Option<usize>>::default();

	let subscription = source.clone().lift_option().subscribe(destination);

	source.next(Some(0));
	source.next(Some(1));
	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_option",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_not_do_anything_when_observing_a_none() {
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Option<usize>>::default();

	let subscription = source.clone().lift_option().subscribe(destination);

	notification_collector.lock().assert_is_empty("lift_option");
	source.next(None);
	notification_collector.lock().assert_is_empty("lift_option");

	source.next(Some(0));
	source.next(None);
	source.next(Some(1));

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_option",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(1),
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Option<usize>, &'static str>::default();

	let subscription = source.clone().lift_option().subscribe(destination);

	let error = "error";
	source.next(Some(0));
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_option",
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
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Option<usize>>::default();

	let subscription = source.clone().lift_option().subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_option",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<Option<usize>>::default();

	let composed = compose_operator::<Option<usize>, Never>().lift_option();

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"lift_option",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}
