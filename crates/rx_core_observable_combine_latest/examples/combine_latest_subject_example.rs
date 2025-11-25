use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockClock;

/// The [CombineLatestObserver] combines the latest values from multiple other
/// observables.
fn main() {
	let mut clock = MockClock::default();
	let mut context = ();

	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut subscription = combine_latest(subject_1.clone(), subject_2.clone())
		.subscribe(PrintObserver::new("combine_latest"), &mut context);

	subject_1.next(1, &mut context);
	subject_2.next(10, &mut context);
	subject_2.next(20, &mut context);

	// The inner RcSubscriber ensures only one tick gets downstream
	subscription.tick(clock.elapse(Duration::from_millis(200)), &mut context);

	subject_1.next(2, &mut context);
	subject_1.next(3, &mut context);

	subject_2.next(30, &mut context);

	subject_1.complete(&mut context); // The first completion won't complete the entire thing
	println!("subject 1 was completed!");
	subject_2.complete(&mut context);
	subscription.unsubscribe(&mut context);
}
