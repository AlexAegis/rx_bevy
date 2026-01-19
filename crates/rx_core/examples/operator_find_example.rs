use rx_core::prelude::*;

/// The [FindOperator] completes immediately when observing the find value
fn main() {
	let _s = (1..=5)
		.into_observable()
		.find(|i| i % 2 == 0)
		.subscribe(PrintObserver::new("find_operator"));
}
