use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_a_single_value_then_complete() {
	let value = 4;
	let mut observable = of(value);
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = observable.subscribe(mock_observer);
	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"of",
		0,
		[
			SubscriberNotification::Next(4),
			SubscriberNotification::Complete,
			SubscriberNotification::Unsubscribe,
		],
		true,
	);
}
