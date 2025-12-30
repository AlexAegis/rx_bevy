use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_map_next_emissions_using_the_mapper_provided() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source
		.clone()
		.map(|i| format!("{}", i))
		.subscribe(destination);

	source.next(0);
	source.next(1);

	assert!(!subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map",
		0,
		[
			SubscriberNotification::Next("0".to_string()),
			SubscriberNotification::Next("1".to_string()),
		],
		true,
	);
}

#[test]
fn should_close_when_errored() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.map(|i| format!("{}", i))
		.subscribe(destination);
	let teardown_tracker = subscription.add_tracked_teardown("map");

	let error = "error";
	source.next(0);
	source.error(error);

	notification_collector.lock().assert_notifications(
		"map",
		0,
		[
			SubscriberNotification::Next("0".to_string()),
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_close_when_completed() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let mut subscription = source
		.clone()
		.map(|i| format!("{}", i))
		.subscribe(destination);

	let teardown_tracker = subscription.add_tracked_teardown("map");

	source.complete();

	notification_collector.lock().assert_notifications(
		"map",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
	assert!(subscription.is_closed());
	teardown_tracker.assert_was_torn_down();
}

#[test]
fn should_compose() {
	let destination = MockObserver::<String, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().map(|i| format!("{}", i));

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"map",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
