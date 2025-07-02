use rx_bevy::prelude::*;

/// The [CombineLatestObserver] combines the latest values from multiple observables
/// Notice that in the output, 1, and 2 is not present, that's because
/// the first observable emits all of its values immediately upon subscription,
/// before the second one could even start listening.
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut _s = combine_latest(subject_1.clone(), subject_2.clone()).subscribe(
		DynFnObserver::default().with_next(|next: (i32, i32)| println!("{}, {}", next.0, next.1)),
	);

	subject_1.next(1);
	subject_2.next(10);
	subject_2.next(20);

	subject_1.next(2);
	subject_1.next(3);

	subject_2.next(30);

	subject_1.complete();
	_s.unsubscribe();
}
