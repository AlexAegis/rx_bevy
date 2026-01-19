use rx_core::prelude::*;

/// The [FindOperator] errors when upstream completes without emitting a value
fn main() {
	let _s = empty()
		.find(|_i| true)
		.subscribe(PrintObserver::new("find_operator"));
}
