use rx_core::prelude::*;
use rx_core_common::SubscriberNotification;
use rx_core_testing::prelude::*;

#[test]
fn should_immediately_unsubscribe() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = closed().subscribe(mock_observer);

	notification_collector.lock().assert_notifications(
		"closed",
		0,
		[SubscriberNotification::Unsubscribe],
		true,
	);

	subscription.unsubscribe();

	notification_collector
		.lock()
		.assert_nth_notification_is_last("closed", 0);
}
