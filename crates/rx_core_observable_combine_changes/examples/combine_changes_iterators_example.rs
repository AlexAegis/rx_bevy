use rx_core::prelude::*;

/// Iterators emit all their values immediately upon subscription! So it's
/// expected that the first three emissions are just from the first observable!
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s = combine_changes(observable_1, observable_2)
		.subscribe(PrintObserver::new("combine_changes"));
}
