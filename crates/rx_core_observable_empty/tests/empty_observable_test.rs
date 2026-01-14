use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_immediately_emit_complete() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = EmptyObservable.subscribe(mock_observer);
	subscription.unsubscribe();

	assert!(matches!(
		notification_collector.lock().nth_notification(0),
		SubscriberNotification::Complete
	));
}

mod observable_fn {
	use rx_core::prelude::empty;

	use super::*;

	#[test]
	fn should_immediately_emit_complete() {
		let mock_observer = MockObserver::default();
		let notification_collector = mock_observer.get_notification_collector();

		let mut subscription = empty().subscribe(mock_observer);
		subscription.unsubscribe();

		assert!(matches!(
			notification_collector.lock().nth_notification(0),
			SubscriberNotification::Complete
		));
	}
}

/// rx_contract_closed_after_error - does not error
mod contracts {
	use rx_core::prelude::empty;

	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<EmptyObservable, Never, Never>::new_with_source("empty", empty());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<EmptyObservable, Never, Never>::new_with_source("empty", empty());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		// Empty emits Complete immediately on subscribe; unsubscribe is a no-op on an already closed subscription.
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}
}
