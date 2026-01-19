use rx_core::prelude::*;

/// The [FindIndexOperator] errors when upstream completes without emitting a value
fn main() {
	let _s = empty()
		.find_index(|_i| true)
		.subscribe(PrintObserver::new("find_index_operator"));
}
