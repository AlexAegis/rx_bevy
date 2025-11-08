use rx_core::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut subscription = merge(subject_1.clone(), subject_2.clone())
		.subscribe(PrintObserver::<i32>::new("merge_operator"), &mut ());

	subject_1.next(1, &mut ());
	subject_2.next(2, &mut ());

	subject_2.complete(&mut ());

	subject_1.next(3, &mut ());

	subject_1.complete(&mut ());
	subscription.unsubscribe(&mut ());
}
