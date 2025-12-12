use rx_core::prelude::*;

/// The [FirstOperator] completes immediately when observing the first value
fn main() {
	let _s = (1..=5)
		.into_observable()
		.first()
		.subscribe(PrintObserver::new("first_operator"));
}
