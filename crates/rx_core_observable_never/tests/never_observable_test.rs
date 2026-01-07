use rx_core::prelude::*;
use rx_core_common::{SubscriberNotification, SubscriptionLike};
use rx_core_testing::prelude::*;

#[test]
fn should_not_emit_anything() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = never().subscribe(mock_observer);
	assert!(notification_collector.lock().is_empty());
	subscription.unsubscribe();
	assert!(matches!(
		notification_collector.lock().nth_notification(0),
		SubscriberNotification::Unsubscribe
	));
	assert_eq!(notification_collector.lock().len(), 1);
}
