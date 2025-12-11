use rx_core::prelude::*;

/// The [ZipObservable] combines values from multiple observables, grouping
/// their emissions in the order they were emitted. That is, the first emission
/// of the first observable will only ever be seen together with the first
/// emission of the second observable. And their second emissions will too appear
/// together and so on.
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut _s = zip(subject_1.clone(), subject_2.clone()).subscribe(
		DynFnObserver::default()
			.with_next(|next: (i32, i32)| println!("zip_next {}, {}", next.0, next.1))
			.with_complete(|| println!("zip_complete"))
			.with_unsubscribe(|| println!("zip_unsubscribe")),
	);

	subject_1.next(1);
	subject_2.next(10);
	subject_2.next(20);

	subject_1.next(2);
	subject_1.next(3);
	// Even though the other subject does not complete, this one does, and since
	// nothing is left in the queue of this observable, no matter what the other
	// observable emits, the zip can no longer emit anything, so it completes.
	subject_1.complete();

	// Even if the last emission of subject 1 was consumed after it was completed!
	subject_2.next(30);
}
