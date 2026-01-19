use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let mut subject = PublishSubject::<i32>::default();

	let mut subscription = subject
		.clone()
		.fallback_when_silent(|_, _, _| Default::default(), scheduler)
		.subscribe(PrintObserver::<i32>::new("fallback_when_silent"));

	subject.next(1);
	executor.tick(Duration::from_millis(200));
	subject.next(2);
	executor.tick(Duration::from_millis(200));
	// Silence
	executor.tick(Duration::from_millis(200));
	subject.next(3);
	executor.tick(Duration::from_millis(200));

	subscription.unsubscribe();
}
