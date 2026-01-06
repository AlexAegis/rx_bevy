use rx_core::prelude::*;
use rx_core_testing::prelude::*;

mod contracts {
	use super::*;

	#[test]
	fn rx_contract_closed_after_error() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new_operator_harness(
				"map_never (next)",
			);
		let observable = harness.create_harness_observable().map_never();
		harness.assert_rx_contract_closed_after_error(observable, TestError, TestError);
	}

	#[test]
	fn rx_contract_closed_after_complete() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new_operator_harness(
				"map_never (next)",
			);
		let observable = harness.create_harness_observable().map_never();
		harness.assert_rx_contract_closed_after_complete(observable);
	}

	#[test]
	fn rx_contract_closed_after_unsubscribe() {
		let mut harness =
			TestHarness::<TestSubject<Never, TestError>, usize, TestError>::new_operator_harness(
				"map_never (next)",
			);
		let observable = harness.create_harness_observable().map_never();
		harness.assert_rx_contract_closed_after_unsubscribe(observable);
	}
}

mod compose {
	use super::*;

	#[test]
	fn should_compose() {
		let mut harness =
			TestHarness::<_, Never, TestError>::new_operator_harness("map_never (next)");
		let composed = compose_operator::<Never, TestError>().map_never();
		let observable = harness.create_harness_observable().pipe(composed);
		harness.assert_rx_contract_closed_after_unsubscribe(observable);
	}
}
