use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_cause_an_error_on_subscribe() {
	let destination = MockObserver::default();
	let notification_collector = destination.get_notification_collector();

	let error = "error";
	let _subscription = throw(error).subscribe(destination);

	notification_collector.lock().assert_notifications(
		"throw",
		0,
		[SubscriberNotification::Error(error)],
		true,
	);
}

/// rx_contract_closed_after_complete - does not complete
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<_, Never, TestError>::new_with_source("throw", throw(TestError));
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<_, Never, TestError>::new_with_source("throw", throw(TestError));
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		// `throw` errors immediately on subscribe; unsubscribe after is a no-op on a closed subscription.
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}
}
