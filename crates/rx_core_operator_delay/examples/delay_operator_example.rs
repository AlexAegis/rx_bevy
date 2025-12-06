use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::{SchedulerWithManualTickElapseExtension, TickingSchedulerExecutor};

/// The [DelaynOperator] re-emits every upstream value after a duration had
/// elapsed.
fn main() {
	let mut executor = TickingSchedulerExecutor::<()>::default();
	let mut scheduler = executor.get_scheduler();
	executor.tick_by_delta(Duration::from_millis(200), &mut ());

	scheduler.get_scheduler().schedule_delayed_task(
		|_| {
			println!("late hello");
			Ok(())
		},
		Duration::from_millis(4000),
	);

	scheduler.get_scheduler().schedule_delayed_task(
		|_| {
			println!("early hello");
			Ok(())
		},
		Duration::from_millis(10),
	);

	executor.tick_by_delta(Duration::from_millis(200), &mut ());

	let _subscription = (1..=5)
		.into_observable::<()>()
		.delay(DelayOperatorOptions {
			delay: Duration::from_millis(1000),
			scheduler: scheduler.clone(),
		})
		.subscribe(PrintObserver::new("delay_operator"), &mut ());

	executor.tick_by_delta(Duration::from_millis(1200), &mut ());
	println!("ticked 1200ms");
	executor.tick_by_delta(Duration::from_millis(4200), &mut ());

	//subscription.tick(clock.elapse(Duration::from_millis(800)), &mut ());
	//subscription.tick(clock.elapse(Duration::from_millis(400)), &mut ());
	//println!("ticked 400ms");
}
