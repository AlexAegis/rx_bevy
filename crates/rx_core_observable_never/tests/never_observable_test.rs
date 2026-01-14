use rx_core::prelude::*;
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

/// rx_contract_closed_after_error - does not error
/// rx_contract_closed_after_complete - does not complete
mod contracts {

	use super::*;
	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<_, Never, Never>::new_with_source("never", never());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
