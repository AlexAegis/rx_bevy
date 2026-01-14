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

	notification_collector.print();

	notification_collector.lock().assert_notifications(
		"iterator",
		0,
		[
			SubscriberNotification::Next(1),
			SubscriberNotification::Next(2),
			SubscriberNotification::Next(3),
			SubscriberNotification::Complete,
		],
		true,
	);

	assert!(subscription.is_closed());
}

/// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<_, usize, Never>::new_with_source("iterator", (1..=2).into_observable());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<_, usize, Never>::new_with_source("iterator", (1..=2).into_observable());
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		// Iterator emits and completes synchronously on subscribe; unsubscribe after that is a no-op.
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}
}
