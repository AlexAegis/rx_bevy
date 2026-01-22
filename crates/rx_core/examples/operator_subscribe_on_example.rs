use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let _subscription = (1..=3)
		.into_observable()
		.subscribe_on(scheduler)
		.subscribe(PrintObserver::new("subscribe_on_operator"));

	executor.tick(Duration::from_millis(0));
}
