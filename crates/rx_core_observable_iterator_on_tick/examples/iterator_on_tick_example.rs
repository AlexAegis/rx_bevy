use std::time::Duration;

use rx_bevy::prelude::*;
use rx_core_testing::MockClock;

/// An [IteratorOnTickObservable] emits a single value from an iterator on every
/// tick. Never use this as a timer as it has no knowledge of how much time has
/// passed, it just emits whenever a tick happens, and that depends on the
/// scheduler being used.
fn main() {
	let mut context = ();

	let iterator_observable = IteratorOnTickObservable::new(
		0..=7,
		OnTickObservableOptions {
			start_on_subscribe: true,
			emit_at_every_nth_tick: 2,
		},
	);
	let mut subscription_handle = iterator_observable
		.finalize(|_| println!("fin"))
		.subscribe(PrintObserver::new("iterator_on_tick"), &mut context);
	println!("subscribed!");

	let mut mock_clock = MockClock::default();
	println!("ticking 500ms...");
	subscription_handle.tick(mock_clock.elapse(Duration::from_millis(500)), &mut context);
	println!("ticking 16ms...");
	subscription_handle.tick(mock_clock.elapse(Duration::from_millis(16)), &mut context);
	println!("ticking 9001ms...");
	subscription_handle.tick(mock_clock.elapse(Duration::from_millis(9001)), &mut context);
	println!("ticking 0ms...");
	subscription_handle.tick(mock_clock.elapse(Duration::from_millis(0)), &mut context);
	println!("ticking 10ms...");
	subscription_handle.tick(mock_clock.elapse(Duration::from_millis(10)), &mut context);
}
