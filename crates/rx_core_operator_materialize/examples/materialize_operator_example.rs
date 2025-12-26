use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.materialize()
		.subscribe(PrintObserver::new("materialize_operator"));
}
