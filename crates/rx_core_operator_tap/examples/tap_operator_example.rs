use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.tap(PrintObserver::new("tap_destination"))
		.subscribe(PrintObserver::new("tap_operator"));
}
