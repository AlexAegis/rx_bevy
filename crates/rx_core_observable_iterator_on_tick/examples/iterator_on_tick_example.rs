use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

/// An [IteratorOnTickObservable] emits a single value from an iterator on every
/// tick. Never use this as a timer as it has no knowledge of how much time has
/// passed, it just emits whenever a tick happens, and that depends on the
/// scheduler being used.
fn main() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let iterator_observable = IteratorOnTickObservable::new(
		0..=7,
		OnTickObservableOptions {
			start_on_subscribe: true,
			emit_at_every_nth_tick: 2,
		},
		scheduler,
	);
	let _subscription = iterator_observable
		.finalize(|| println!("fin"))
		.subscribe(PrintObserver::new("iterator_on_tick"));
	println!("subscribed!");

	executor.tick_by_delta(Duration::from_millis(500));
	executor.tick_by_delta(Duration::from_millis(16));
	executor.tick_by_delta(Duration::from_millis(9001));
	executor.tick_by_delta(Duration::from_millis(0));
	executor.tick_by_delta(Duration::from_millis(10));
}
