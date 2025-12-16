use rx_core::prelude::*;

/// The [ZipObservable] combines values from multiple observables, grouping
/// their emissions in the order they were emitted. That is, the first emission
/// of the first observable will only ever be seen together with the first
/// emission of the second observable. And their second emissions will too appear
/// together and so on.
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s = zip(observable_1, observable_2)
		.with_options(ZipSubscriberOptions {
			max_queue_length: 2, // Since the first observable immediately fires all 3 of its values, the first emission `1` will be dropped, and nothing will pair up with 6
			overflow_behavior: QueueOverflowBehavior::DropOldest,
		})
		.subscribe(PrintObserver::new("zip_backpressure"));
}
