use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let mut harness = TestHarness::<_, Never, TestError>::new("map_never (next)");
		let composed = compose_operator::<Never, TestError>().map_never();
		let observable = harness.create_harness_observable().pipe(composed);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new("map_never (next)");
		let observable = harness.create_harness_observable().map_never();
		harness.subscribe_to(observable);
		harness.source().error(TestError);
		harness.assert_terminal_notification(SubscriberNotification::Error(TestError));
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new("map_never (next)");
		let observable = harness.create_harness_observable().map_never();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new("map_never (next)");
		let observable = harness.create_harness_observable().map_never();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
