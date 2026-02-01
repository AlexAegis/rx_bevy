use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let mut trigger = PublishSubject::<bool>::default();

	let scheduler_clone = scheduler.clone();
	let _subscription = trigger
		.clone()
		.switch_map(
			move |is_silence| {
				if is_silence {
					never().map_never_both::<usize, Never>().erase()
				} else {
					interval(
						IntervalObservableOptions {
							duration: Duration::from_millis(1),
							max_emissions_per_tick: 1000,
							start_on_subscribe: false,
						},
						scheduler_clone.clone(),
					)
					.map(|i| i + 1)
					.map_never()
					.erase()
				}
			},
			Never::map_into(),
		)
		.throttle_time(
			ThrottleTimeOptions::new(Duration::from_millis(500)),
			scheduler,
		)
		.subscribe(PrintObserver::new("throttle_time_operator"));

	trigger.next(false);
	for _ in 0..10 {
		executor.tick(Duration::from_millis(100));
	}
	trigger.next(true);
	// Waiting more than the throttling window, but not divisible by it.
	// Throttling starts when a new value is observed without already actively
	// throttling, it does not simply quantize them over time.
	executor.tick(Duration::from_millis(700));

	trigger.next(false);
	for _ in 0..10 {
		executor.tick(Duration::from_millis(100));
	}
}
