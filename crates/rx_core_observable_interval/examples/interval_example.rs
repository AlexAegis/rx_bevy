use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let mut mock_executor = MockExecutor::new_with_logging();
	let scheduler = mock_executor.get_scheduler_handle();

	let mut interval_observable = IntervalObservable::new(
		IntervalObservableOptions {
			duration: Duration::from_secs(1),
			max_emissions_per_tick: 3,
			start_on_subscribe: true,
		},
		scheduler,
	);
	let _subscription = interval_observable.subscribe(PrintObserver::new("interval_observable"));

	mock_executor.tick(Duration::from_millis(600));
	mock_executor.tick(Duration::from_millis(401));
	mock_executor.tick(Duration::from_millis(16200)); // lag spike! would result in 16 emissions, but the limit is 2!
	mock_executor.tick(Duration::from_millis(1200));
	mock_executor.tick(Duration::from_millis(2200));
}
