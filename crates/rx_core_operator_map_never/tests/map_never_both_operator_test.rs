use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new("map_never (both)");
		let composed = compose_operator::<Never, Never>().map_never_both::<usize, TestError>();
		let observable = harness.create_harness_observable().pipe(composed);
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}

/// rx_contract_closed_after_next - does not next
/// rx_contract_closed_after_error - does not error
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new("map_never (both)");
		let observable = harness.create_harness_observable().map_never_both();
		harness.subscribe_to(observable);
		harness.source().complete();
		harness.assert_terminal_notification(SubscriberNotification::Complete);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new("map_never (both)");
		let observable = harness.create_harness_observable().map_never_both();
		harness.subscribe_to(observable);
		harness.get_subscription_mut().unsubscribe();
		harness.assert_terminal_notification(SubscriberNotification::Unsubscribe);
	}
}
