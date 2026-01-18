use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::default();
	let scheduler = executor.get_scheduler_handle();

	let envelope = AdsrEnvelope {
		attack_time: Duration::from_millis(10),
		decay_time: Duration::from_millis(10),
		sustain_volume: 0.5,
		release_time: Duration::from_millis(15),
		..Default::default()
	};

	let mut source = PublishSubject::<AdsrTrigger>::default();

	let mut subscription = source
		.clone()
		.adsr(
			AdsrOperatorOptions {
				envelope,
				..Default::default()
			},
			scheduler.clone(),
		)
		.subscribe(PrintObserver::new("adsr"));

	source.next(true.into());
	executor.tick(Duration::from_millis(10));
	executor.tick(Duration::from_millis(10));

	source.next(false.into());
	executor.tick(Duration::from_millis(15));

	subscription.unsubscribe();
}
