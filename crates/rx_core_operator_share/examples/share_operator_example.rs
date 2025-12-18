use std::time::Duration;

use rx_core::prelude::*;
use rx_core_testing::MockExecutor;

fn main() {
	let mut executor = MockExecutor::new_with_logging();
	let scheduler = executor.get_scheduler_handle();
	let shared_interval = interval(
		IntervalObservableOptions {
			duration: Duration::from_secs(1),
			max_emissions_per_tick: 10,
			..Default::default()
		},
		scheduler,
	)
	.finalize(|| println!("shared interval: unsubscribed"))
	.tap_next(|n| println!("shared interval next: {n}"))
	.share(ShareOptions {
		connector_creator: || PublishSubject::default(),
		reset_on_complete: true,
	});

	// No subscriptions yet, these will not advance the interval as there isn't one
	executor.tick(Duration::from_secs(7));

	let _s1 = shared_interval
		.clone()
		.subscribe(PrintObserver::new("share_operator_1"));

	// A subscription was established, now that share is hot, there is an active interval subscription!
	executor.tick(Duration::from_secs(4));

	let _s2 = shared_interval
		.clone()
		.subscribe(PrintObserver::new("share_operator_2"));

	// A subscription was already hot, the same interval output is received by the second subscription too
	executor.tick(Duration::from_secs(2));
}
