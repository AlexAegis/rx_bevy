use rx_bevy::prelude::*;

/// The [ZipObservable] combines values from multiple observables, grouping
/// their emissions in the order they were emitted. That is, the first emission
/// of the first observable will only ever be seen together with the first
/// emission of the second observable. And their second emissions will too appear
/// together and so on.
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut _s = zip(subject_1.clone(), subject_2.clone()).subscribe(
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
