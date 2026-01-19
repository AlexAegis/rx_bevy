use rx_core::prelude::*;

/// The [EndWithOperator] emits a value right before completion
fn main() {
	let _s = (1..=5)
		.into_observable()
		.end_with(99)
		.subscribe(PrintObserver::new("end_with_operator"));
}
