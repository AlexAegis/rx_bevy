use rx_core::prelude::*;
use rx_core_testing::prelude::*;

/// rx_contract_closed_after_next - impossible, in type is never
/// rx_contract_closed_after_error - impossible, error type is never
mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new_operator_harness(
				"map_never (both)",
			);
		let observable = harness.create_harness_observable().map_never_both();
		harness.assert_rx_contract_closed_after_complete(observable);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new_operator_harness(
				"map_never (both)",
			);
		let observable = harness.create_harness_observable().map_never_both();
		harness.assert_rx_contract_closed_after_unsubscribe(observable);
	}
}

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let mut harness =
			TestHarness::<TestSubject<Never, Never>, usize, TestError>::new_operator_harness(
				"map_never (both)",
			);
		let composed = compose_operator::<Never, Never>().map_never_both::<usize, TestError>();
		let observable = harness.create_harness_observable().pipe(composed);
		harness.assert_rx_contract_closed_after_unsubscribe(observable);
	}
}
