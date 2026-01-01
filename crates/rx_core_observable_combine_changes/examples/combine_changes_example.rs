use rx_core::prelude::*;

fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s = combine_changes(observable_1, observable_2)
		.subscribe(PrintObserver::new("combine_changes"));
}
