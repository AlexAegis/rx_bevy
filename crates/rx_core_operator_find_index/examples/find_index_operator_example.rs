use rx_core::prelude::*;

/// The [FindIndexOperator] completes immediately when observing a value matched
/// by the predicate, emitting the index of the value found.
fn main() {
	let _s = (1..=5)
		.into_observable()
		.find_index(|i| i % 2 == 0)
		.subscribe(PrintObserver::new("find_index_operator"));
}
