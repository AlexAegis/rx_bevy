use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let mut subject = PublishSubject::<usize>::default();

	let _subscription = subject
		.clone()
		.debounce_time(Duration::from_millis(1000), scheduler)
		.subscribe(PrintObserver::new("debounce_time_operator"));

	subject.next(1);
	executor.tick(Duration::from_millis(500));
	subject.next(2);
	executor.tick(Duration::from_millis(1000));
	subject.complete();
}
