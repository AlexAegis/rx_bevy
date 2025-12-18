use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut mock_executor = MockExecutor::new_with_logging();
	let scheduler = mock_executor.get_scheduler_handle();

	let mut timer_observable = TimerObservable::new(Duration::from_secs(1), scheduler);
	let _subscription = timer_observable.subscribe(PrintObserver::new("timer_observable"));

	mock_executor.tick(Duration::from_millis(600));
	mock_executor.tick(Duration::from_millis(400));
}
