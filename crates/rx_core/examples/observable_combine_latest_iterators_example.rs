use rx_core::prelude::*;

/// The [CombineLatestObserver] combines the latest values from multiple observables
/// Notice that in the output, 1, and 2 is not present, that's because
/// the first observable emits all of its values immediately upon subscription,
/// before the second emits something to pair it up with!
///
/// Check out the `combine_changes_iterators_example` to see this behavior in
/// action better!
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s =
		combine_latest(observable_1, observable_2).subscribe(PrintObserver::new("combine_latest"));
}
