use rx_core::prelude::*;
use rx_core_common::Never;
use rx_core_testing::prelude::*;

#[test]
fn should_emit_a_single_value_and_stay_open_when_no_terminal_signals_were_sent() {
	let mock_observer = MockObserver::default();
	let notification_collector = mock_observer.get_notification_collector();

	let subscription = create_observable::<_, Never, _>(|destination| {
		destination.next(1);
	})
	.subscribe(mock_observer);

	notification_collector.lock().assert_notifications(
		"create",
		0,
		[SubscriberNotification::Next(1)],
		true,
	);

	assert!(!subscription.is_closed());
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness = TestHarness::<_, usize, Never>::new_with_source(
			"create",
			create_observable::<_, Never, _>(|destination| {
				destination.next(1);
				destination.complete();
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness = TestHarness::<_, usize, TestError>::new_with_source(
			"create",
			create_observable::<_, TestError, _>(|destination| {
				destination.error(TestError);
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness = TestHarness::<_, usize, Never>::new_with_source(
			"create",
			create_observable::<_, Never, _>(|destination| {
				destination.next(1);
			}),
		);
		let observable = harness.create_harness_observable();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
