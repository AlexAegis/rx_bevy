use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

/// The [DelayOperator] re-emits every upstream value after a duration had
/// elapsed.
fn main() {
	let mut executor = MockExecutor::default();
	let mut scheduler = executor.get_scheduler_handle();
	let cancel_id = scheduler.lock().generate_cancellation_id();

	let mut subscription = (1..=5)
		.into_observable()
		.map(|i| i * 2)
		.delay(DelayOperatorOptions {
			delay: Duration::from_millis(1000),
			scheduler: scheduler.clone(),
		})
		.subscribe(PrintObserver::new("delay_operator"));

	let mut scheduler_clone = scheduler.clone();
	scheduler.lock().schedule_delayed_task(
		move |_, _| {
			println!("late hello");
		},
		Duration::from_millis(4000),
		cancel_id,
	);

	scheduler.clone().lock().schedule_delayed_task(
		move |_, _| {
			println!("early hello");
			scheduler_clone.lock().schedule_immediate_task(
				|_, _| {
					println!("immediate");
				},
				cancel_id,
			);
		},
		Duration::from_millis(10),
		cancel_id,
	);

	subscription.unsubscribe();
	println!("unsubscribed");

	executor.tick_by_delta(Duration::from_millis(200));
	executor.tick_by_delta(Duration::from_millis(1200));
}
