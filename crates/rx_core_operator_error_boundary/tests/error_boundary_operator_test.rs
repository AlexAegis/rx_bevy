use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_next_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize>::default();

	let _subscription = source.clone().error_boundary().subscribe(destination);

	source.next(10);
	source.next(20);
	source.next(30);

	notification_collector.lock().assert_notifications(
		"error_boundary",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(20),
			SubscriberNotification::Next(30),
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize>::default();

	let _subscription = source.clone().error_boundary().subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"error_boundary",
		0,
		[SubscriberNotification::Complete],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, Never>::default();

	let composed = compose_operator::<usize, Never>().error_boundary();

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(10);
	source.next(20);
	source.next(30);

	notification_collector.lock().assert_notifications(
		"error_boundary",
		0,
		[
			SubscriberNotification::Next(10),
			SubscriberNotification::Next(20),
			SubscriberNotification::Next(30),
		],
		true,
	);
}
