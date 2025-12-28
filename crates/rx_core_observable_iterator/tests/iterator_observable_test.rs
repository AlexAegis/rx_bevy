use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn iterator_observable_should_emit_its_values_then_complete() {
	let mock_destination = MockObserver::default();
	let notification_collector = mock_destination.get_notification_collector();

	let subscription = (1..=2).into_observable().subscribe(mock_destination);

	notification_collector.lock().assert_notifications(
		"iterator",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}

#[test]
fn iterator_observable_should_stop_when_downstream_closes_during_iteration() {
	let mock_destination = MockObserver::default();
	let notification_collector = mock_destination.get_notification_collector();

	let subscription = (1..=10)
		.into_observable()
		.take(3)
		.subscribe(mock_destination);

	notification_collector.lock().assert_notifications(
		"iterator",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);

	assert!(subscription.is_closed());
}
