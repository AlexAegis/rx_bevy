use rx_bevy::prelude::*;
use rx_bevy_ref_observable_combine_latest::combine_latest;

/// The [CombineLatestObserver] combines the latest values from multiple observables
/// Notice that in the output, 1, and 2 is not present, that's because
/// the first observable emits all of its values immediately upon subscription,
/// before the second one could even start listening.
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s = combine_latest(observable_1, observable_2).subscribe(
		DynFnObserver::default()
			.with_next(|next: (i32, i32), _context| println!("{}, {}", next.0, next.1)),
		&mut (),
	);
}
