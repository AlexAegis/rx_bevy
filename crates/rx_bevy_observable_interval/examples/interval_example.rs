use std::time::Duration;

use rx_bevy::prelude::*;
use rx_bevy_observable_interval::{IntervalObservable, IntervalObservableOptions};
use rx_bevy_testing::MockClock;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let mut mock_clock = MockClock::default();
	let mut context = ();

	let mut interval_observable = IntervalObservable::new(IntervalObservableOptions {
		duration: Duration::from_secs(1),
		start_on_subscribe: true,
		max_emissions_per_tick: 3,
	});
	let mut subscription =
		interval_observable.subscribe(PrintObserver::new("interval_observable"), &mut context);

	println!("subscribed!");
	subscription.tick(mock_clock.elapse(Duration::from_millis(600)), &mut context);
	println!("600ms elapsed!");
	subscription.tick(mock_clock.elapse(Duration::from_millis(400)), &mut context);
	println!("400ms elapsed!");
	subscription.tick(
		mock_clock.elapse(Duration::from_millis(16200)),
		&mut context,
	); // lag spike! would result in 16 emissions, but the limit is 2!
	println!("16200ms elapsed!");
	subscription.tick(mock_clock.elapse(Duration::from_millis(1200)), &mut context);
	println!("1200ms elapsed!");
	subscription.tick(mock_clock.elapse(Duration::from_millis(2200)), &mut context);
	println!("2200ms elapsed!");
}
