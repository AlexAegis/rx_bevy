use rx_core::prelude::*;
use rx_core_common::{Never, SubscriberNotification, WorkExecutor};
use rx_core_testing::prelude::*;

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, Never>, AdsrSignal, Never>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.source().complete();

		harness.assert_terminal_notification(SubscriberNotification::Complete);
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_error() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, TestError>, AdsrSignal, TestError>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.source().error(TestError);

		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
		assert!(executor.is_empty());
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let executor = MockExecutor::default();
		let scheduler = executor.get_scheduler_handle();
		let mut harness =
			TestHarness::<TestSubject<AdsrTrigger, Never>, AdsrSignal, Never>::new("adsr");

		let observable = harness
			.create_harness_observable()
			.adsr(AdsrOperatorOptions::default(), scheduler.clone());
		harness.subscribe_to(observable);

		harness.source().next(AdsrTrigger {
			activated: true,
			envelope_changes: None,
		});
		harness.get_subscription_mut().unsubscribe();

		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
		assert!(executor.is_empty());
	}
}
