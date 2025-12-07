use std::time::Duration;

use rx_core::{prelude::*, scheduler};
use rx_core_testing::{MockClock, MockExecutor};

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let mut mock_executor = MockExecutor::default();
	let scheduler = mock_executor.get_scheduler();

	let mut interval_observable = IntervalObservable::<()>::new(IntervalObservableOptions {
		duration: Duration::from_secs(1),
		scheduler,
		max_emissions_per_tick: 2,
		start_on_subscribe: true,
	});
	let _subscription =
		interval_observable.subscribe(PrintObserver::new("interval_observable"), &mut ());

	println!("subscribed!");
	println!("elapsing 600ms...");
	mock_executor.tick_by_delta(Duration::from_millis(600));
	mock_executor.tick_by_delta(Duration::from_millis(400));
	println!("elapsing 400ms...");
	mock_executor.tick_by_delta(Duration::from_millis(16200)); // lag spike! would result in 16 emissions, but the limit is 2!
	println!("elapsing 16200ms...");
	mock_executor.tick_by_delta(Duration::from_millis(1200));
	println!("elapsing 1200ms...");
	mock_executor.tick_by_delta(Duration::from_millis(2200));
	println!("elapsing 2200ms...");
}
