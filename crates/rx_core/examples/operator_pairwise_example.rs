use rx_core::prelude::*;

fn main() {
	let _s = (1..=4)
		.into_observable()
		.pairwise()
		.subscribe(PrintObserver::new("pairwise_operator"));
}
