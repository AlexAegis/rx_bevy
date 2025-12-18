use rx_core::prelude::*;

/// The [StartWithOperator] emits a value immediately on subscription
fn main() {
	let _s = (1..=5)
		.into_observable()
		.start_with(Default::default)
		.subscribe(PrintObserver::new("start_with_operator"));
}
