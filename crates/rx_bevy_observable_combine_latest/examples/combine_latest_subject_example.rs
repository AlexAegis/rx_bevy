use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
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
