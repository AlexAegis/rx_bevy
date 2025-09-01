use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut _s = merge(subject_1.clone(), subject_2.clone())
		.subscribe(PrintObserver::<i32, ()>::new("merge_operator"));

	subject_1.next(1);
	subject_2.next(2);

	subject_2.complete();

	subject_1.next(3);

	subject_1.complete();
	_s.unsubscribe();
}
