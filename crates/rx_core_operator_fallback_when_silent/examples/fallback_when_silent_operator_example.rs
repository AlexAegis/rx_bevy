use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockClock;

fn main() {
	let mut mock_clock = MockClock::default();
	let mut context = ();
	let mut subject = Subject::<i32>::default();

	let mut subscription = subject
		.clone()
		.fallback_when_silent(Default::default)
		.subscribe(PrintObserver::<i32>::new("into_operator"), &mut context);

	subject.next(1, &mut context);
	subscription.tick(mock_clock.elapse(Duration::from_millis(200)), &mut context);
	subject.next(2, &mut context);
	subscription.tick(mock_clock.elapse(Duration::from_millis(200)), &mut context);
	// Silence
	subscription.tick(mock_clock.elapse(Duration::from_millis(200)), &mut context);
	subject.next(3, &mut context);
	subscription.tick(mock_clock.elapse(Duration::from_millis(200)), &mut context);
}
