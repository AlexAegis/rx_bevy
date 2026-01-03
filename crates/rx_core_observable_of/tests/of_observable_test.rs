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
			SubscriberNotification::Unsubscribe,
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
				SubscriberNotification::Unsubscribe,
			],
			true,
		);

		tracked_teardown.assert_was_torn_down();
		inner_teardown_tracker.assert_was_torn_down();

		assert!(subscription.is_closed())
	}
}
