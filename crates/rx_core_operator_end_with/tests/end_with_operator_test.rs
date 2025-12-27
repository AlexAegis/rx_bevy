use rx_core::prelude::*;
use rx_core_testing::prelude::*;
use rx_core_traits::{Observable, SubscriberNotification};

#[test]
fn should_emit_the_end_with_value_right_before_completion() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().end_with(99).subscribe(destination);

	source.next(0);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"end_with",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(99),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_not_emit_the_end_with_value_when_unsubscribed_without_completion() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().end_with(99).subscribe(destination);

	source.next(0);
	source.unsubscribe();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"end_with",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_not_emit_the_end_with_value_when_errored() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let subscription = source.clone().end_with(99).subscribe(destination);

	let error = "error";
	source.error(error);
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"end_with",
		0,
		[
			SubscriberNotification::Error(error),
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}

#[test]
fn should_compose() {
	let destination = MockObserver::<usize, &'static str>::default();
	let notification_collector = destination.get_notification_collector();

	let mut source = PublishSubject::<usize, &'static str>::default();

	let composed = compose_operator::<usize, &'static str>().end_with(99);

	let subscription = source.clone().pipe(composed).subscribe(destination);

	source.next(0);
	source.complete();
	assert!(subscription.is_closed());

	notification_collector.lock().assert_notifications(
		"end_with",
		0,
		[
			SubscriberNotification::Next(0),
			SubscriberNotification::Next(99),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
