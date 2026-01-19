use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();

	let mut subject = AsyncSubject::<i32>::new(|acc, next| acc + next);

	subject.next(1);
	subject.next(2);
	subject.next(3);
	subject.complete();

	let mut _subscription_1 = subject
		.clone()
		.delay(Duration::from_millis(1000), scheduler)
		.subscribe(PrintObserver::<i32>::new("async_subject"));

	executor.tick(Duration::from_millis(1000));
	println!("end");
}
