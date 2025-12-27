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
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
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
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
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
