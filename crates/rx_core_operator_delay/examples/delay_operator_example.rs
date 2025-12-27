use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

/// The [DelayOperator] re-emits every upstream value after a duration had
/// elapsed.
fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let _subscription = (1..=5)
		.into_observable()
		.map(|i| i * 2)
		.delay(Duration::from_millis(1000), scheduler.clone())
		.subscribe(PrintObserver::new("delay_operator"));

	executor.tick(Duration::from_millis(200));
	executor.tick(Duration::from_millis(1200));
}
