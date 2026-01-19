use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut mock_executor = MockExecutor::new_with_logging();
	let scheduler = mock_executor.get_scheduler_handle();
	let mut enqueue_timer_of_length = PublishSubject::<usize>::default();

	let mut subscription = enqueue_timer_of_length
		.clone()
		.finalize(|| println!("finalize: upstream"))
		.tap_next(|n| println!("emit (source): {n:?}"))
		.map(move |next| {
			interval(
				IntervalObservableOptions {
					duration: Duration::from_secs(1),
					start_on_subscribe: false,
					max_emissions_per_tick: 10,
				},
				scheduler.clone(),
			)
			.finalize(move || println!("timer of {next} finished!"))
			.take(next)
			.map(move |i| format!("{i} (Timer of {next})"))
		})
		.concat_all(Never::map_into())
		.finalize(|| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("concat_all"));

	enqueue_timer_of_length.next(4);
	enqueue_timer_of_length.next(1);
	enqueue_timer_of_length.next(3);
	enqueue_timer_of_length.complete();
	mock_executor.tick(Duration::from_secs(4));
	mock_executor.tick(Duration::from_secs(1));
	mock_executor.tick(Duration::from_secs(3));
	subscription.unsubscribe();
	println!("end");
}
