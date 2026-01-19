use rx_core::prelude::*;

fn main() {
	let mut subject_1 = PublishSubject::<i32>::default();
	let mut subject_2 = PublishSubject::<i32>::default();

	let mut subscription = merge((subject_1.clone(), subject_2.clone()), usize::MAX)
		.subscribe(PrintObserver::<i32>::new("merge_operator"));

	subject_1.next(1);
	subject_2.next(2);

	subject_2.complete();

	subject_1.next(3);

	subject_1.complete();
	subscription.unsubscribe();
}
