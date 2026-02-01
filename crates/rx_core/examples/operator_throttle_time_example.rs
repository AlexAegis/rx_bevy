use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let _subscription = interval(
		IntervalObservableOptions {
			duration: Duration::from_millis(1),
			max_emissions_per_tick: 1000,
			..Default::default()
		},
		scheduler.clone(),
	)
	.throttle_time(
		ThrottleTimeOptions::new(Duration::from_millis(500)),
		scheduler,
	)
	.subscribe(PrintObserver::new("throttle_time_operator"));

	for _ in 0..10 {
		executor.tick(Duration::from_millis(100));
	}
}
