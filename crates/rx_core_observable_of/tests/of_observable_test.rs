use rx_core::prelude::*;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_a_single_value_then_complete() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let mut subscription = of(1).subscribe(mock_observer);
	subscription.unsubscribe();

	notification_collector.lock().assert_notifications(
		"of",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Complete,
		],
		true,
	);
}

mod teardown {
	use super::*;

	#[test]
	fn should_execute_teardowns_on_unsubscribe() {
		let mock_observer = MockObserver::default();
		let notification_collector = mock_observer.get_notification_collector();

		let mut tracked_subscription = SharedSubscription::default();
		let inner_teardown_tracker = tracked_subscription.add_tracked_teardown("of - inner");

		let mut subscription = of(1)
			.finalize(move || {
				tracked_subscription.unsubscribe();
			})
			.subscribe(mock_observer);
		let tracked_teardown = subscription.add_tracked_teardown("of - outer");

		notification_collector.lock().assert_notifications(
			"of",
			0,
			[
				SubscriberNotification::Next(1),
				SubscriberNotification::Complete,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		inner_teardown_tracker.assert_was_torn_down();

		assert!(subscription.is_closed())
	}
}

/// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<_, usize, Never>::new_with_source("of", of(1));
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<_, usize, Never>::new_with_source("of", of(1));
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		// `of` emits synchronously (next + complete); unsubscribe after that is a no-op on a closed subscription.
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}
}
