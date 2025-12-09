use std::time::Duration;

use rx_core::{prelude::*, scheduler};
use rx_core_testing::MockExecutor;

/// An [IteratorOnTickObservable] emits a single value from an iterator on every
/// tick. Never use this as a timer as it has no knowledge of how much time has
/// passed, it just emits whenever a tick happens, and that depends on the
/// scheduler being used.
fn main() {
	let executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let iterator_observable = IteratorOnTickObservable::new(
		0..=7,
		OnTickObservableOptions {
			start_on_subscribe: true,
			emit_at_every_nth_tick: 2,
			scheduler,
		},
	);
	let mut subscription_handle = iterator_observable
		.finalize(|| println!("fin"))
		.subscribe(PrintObserver::new("iterator_on_tick"));
	println!("subscribed!");

	println!("ticking 500ms...");
	executor.tick(mock_clock.elapse(Duration::from_millis(500)));
	println!("ticking 16ms...");
	executor.tick(mock_clock.elapse(Duration::from_millis(16)));
	println!("ticking 9001ms...");
	executor.tick(mock_clock.elapse(Duration::from_millis(9001)));
	println!("ticking 0ms...");
	executor.tick(mock_clock.elapse(Duration::from_millis(0)));
	println!("ticking 10ms...");
	executor.tick(mock_clock.elapse(Duration::from_millis(10)));
}
