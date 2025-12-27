use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let mut source = PublishSubject::<i32>::default();

	let mut subscription = source
		.clone()
		.exhaust_map(
			move |next| {
				println!("Trying to switch to the {}. inner observable..", next);
				interval(
					IntervalObservableOptions {
						duration: Duration::from_millis(1000),
						max_emissions_per_tick: 10,
						start_on_subscribe: false,
					},
					scheduler.clone(),
				)
				.take(3)
			},
			Never::error_mapper(),
		)
		.subscribe(PrintObserver::new("exhaust_map"));

	source.next(1);
	executor.tick(Duration::from_millis(1000));
	executor.tick(Duration::from_millis(1000));
	source.next(2); // Nothing because the inner one hasn't completed yet!
	executor.tick(Duration::from_millis(1000));
	source.next(3); // Switches because `take(3)` completed the inner observable
	source.next(4); // Doesn't switch because the previous one just started!
	executor.tick(Duration::from_millis(1000));
	executor.tick(Duration::from_millis(1000));
	source.complete();
	executor.tick(Duration::from_millis(1000));

	source.unsubscribe();

	println!("end");

	subscription.unsubscribe();
}
