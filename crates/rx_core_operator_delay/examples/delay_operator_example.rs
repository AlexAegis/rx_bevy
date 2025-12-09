use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

/// The [DelaynOperator] re-emits every upstream value after a duration had
/// elapsed.
fn main() {
	let mut executor = MockExecutor::default();
	let mut scheduler = executor.get_scheduler();
	let owner_id = scheduler.lock().generate_owner_id();

	let mut subscription = (1..=5)
		.into_observable::<()>()
		.map(|i| i * 2)
		.delay(DelayOperatorOptions {
			delay: Duration::from_millis(1000),
			scheduler: scheduler.clone(),
		})
		.subscribe(PrintObserver::new("delay_operator"), &mut ());

	let mut scheduler_clone = scheduler.clone();
	scheduler.lock().schedule_delayed_task(
		move |_| {
			println!("late hello");

			Ok(())
		},
		Duration::from_millis(4000),
		owner_id,
	);

	scheduler.clone().lock().schedule_delayed_task(
		move |_| {
			println!("early hello");
			scheduler_clone.lock().schedule_immediate_task(
				|_| {
					println!("immediate");
					Ok(())
				},
				owner_id,
			);
			Ok(())
		},
		Duration::from_millis(10),
		owner_id,
	);

	subscription.unsubscribe(&mut ());
	println!("unsubscribed");

	executor.tick_by_delta(Duration::from_millis(200));
	println!("ticked 200ms");
	executor.tick_by_delta(Duration::from_millis(1200));

	println!("ticked 1200ms");
}
