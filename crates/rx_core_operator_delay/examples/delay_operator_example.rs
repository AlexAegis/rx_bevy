use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::{MockClock, TickingScheduler};

/// The [DelaynOperator] re-emits every upstream value after a duration had
/// elapsed.
fn main() {
	let mut scheduler = TickingScheduler::<()>::default().into_handle();
	scheduler.tick(Duration::from_millis(200), &mut ());

	let mut clock = MockClock::default();
	let mut subscription = (1..=5)
		.into_observable::<()>()
		.delay(DelayOperatorOptions {
			delay: Duration::from_millis(1000),
			scheduler: scheduler.clone(),
		})
		.subscribe(PrintObserver::new("delay_operator"), &mut ());

	subscription.tick(clock.elapse(Duration::from_millis(800)), &mut ());
	println!("ticked 800ms");
	subscription.tick(clock.elapse(Duration::from_millis(400)), &mut ());
	println!("ticked 400ms");
}
