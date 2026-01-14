use rx_core::prelude::*;
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

/// rx_contract_closed_after_error - does not error
/// rx_contract_closed_after_complete - does not complete
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<ClosedObservable, Never, Never>::new_with_source("closed", closed());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
