use rx_core::prelude::*;

/// The [FirstOperator] errors when upstream completes without emitting a value
fn main() {
	let _s = empty()
		.first()
		.subscribe(PrintObserver::new("first_operator"));
}
