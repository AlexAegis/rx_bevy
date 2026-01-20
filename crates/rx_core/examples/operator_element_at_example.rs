use rx_core::prelude::*;

/// The [ElementAtOperator] emits the value at a given zero-based index,
/// then completes immediately.
fn main() {
	let _s = vec!["a", "b", "c", "d", "e"]
		.into_observable()
		.element_at(2)
		.subscribe(PrintObserver::new("element_at_operator"));
}
