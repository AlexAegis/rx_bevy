use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::Observable;

#[test]
fn should_emit_partial_results() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.scan(|acc, next| acc + next, 0)
		.subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"scan",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_complete_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.scan(|acc, next| acc + next, 0)
		.subscribe(destination);

	source.complete();

	notification_collector.lock().assert_notifications(
		"scan",
		0,
		[
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_error_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();
	let _subscription = source
		.clone()
		.scan(|acc, next| acc + next, 0)
		.subscribe(destination);

	let error = "error";
	source.error(error);

	notification_collector.lock().assert_notifications(
		"scan",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_unsubscribe_normally() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let source = PublishSubject::<usize, &'static str>::default();
	let mut subscription = source
		.clone()
		.scan(|acc, next| acc + next, 0)
		.subscribe(destination);

	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"scan",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator().scan(|acc, next| acc + next, 0);

	let _subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(1);
	source.next(2);
	source.next(3);
	source.complete();

	notification_collector.lock().assert_notifications(
		"scan",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(3),
			SubscriberNotification::Next(6),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
