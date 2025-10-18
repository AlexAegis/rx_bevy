use std::time::Duration;

use rx_core::prelude::*;
use rx_core_observable_interval::{IntervalObservable, IntervalObservableOptions};
use rx_core_testing::MockClock;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let mut mock_clock = MockClock::default();
	let mut context = ();

	let mut interval_observable = IntervalObservable::<()>::new(IntervalObservableOptions {
		duration: Duration::from_secs(1),
		start_on_subscribe: false,
		max_emissions_per_tick: 3,
	});
	let mut subscription =
		interval_observable.subscribe(PrintObserver::new("interval_observable"), &mut context);

	println!("subscribed!");
	println!("elapsing 600ms...");
	subscription.tick(mock_clock.elapse(Duration::from_millis(600)), &mut context);
	subscription.tick(mock_clock.elapse(Duration::from_millis(400)), &mut context);
	println!("elapsing 400ms...");
	subscription.tick(
		mock_clock.elapse(Duration::from_millis(16200)),
		&mut context,
	); // lag spike! would result in 16 emissions, but the limit is 2!
	println!("elapsing 16200ms...");
	subscription.tick(mock_clock.elapse(Duration::from_millis(1200)), &mut context);
	println!("elapsing 1200ms...");
	subscription.tick(mock_clock.elapse(Duration::from_millis(2200)), &mut context);
	println!("elapsing 2200ms...");
}
