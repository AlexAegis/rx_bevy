use rx_bevy::prelude::*;
use rx_bevy_ref_observable_combine_latest::combine_latest;

/// The [CombineLatestObserver] combines the latest values from multiple observables
/// Notice that in the output, 1, and 2 is not present, that's because
/// the first observable emits all of its values immediately upon subscription,
/// before the second one could even start listening.
fn main() {
	// TODO: Fix, something gets dropped early and it doesn't emit anything
	let mut subject_1 = Subject::<i32, ()>::default();
	let mut subject_2 = Subject::<i32, ()>::default();

	let mut subscription = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(
		DynFnObserver::default()
			.with_next(|next: (i32, i32), _context| println!("{}, {}", next.0, next.1)),
		&mut (),
	);

	subject_1.next(1, &mut ());
	subject_2.next(10, &mut ());
	subject_2.next(20, &mut ());

	subject_1.next(2, &mut ());
	subject_1.next(3, &mut ());

	subject_2.next(30, &mut ());

	subject_1.complete(&mut ());
	subscription.unsubscribe(&mut ());
}
