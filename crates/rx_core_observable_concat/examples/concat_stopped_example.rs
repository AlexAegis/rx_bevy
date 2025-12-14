use rx_core::prelude::*;

use rx_core_observable_concat::observable_fn::concat;

fn main() {
	let mut subject_1 = Subject::<usize>::default();
	let subject_2 = Subject::<usize>::default();
	let mut subject_3 = Subject::<usize>::default();

	let _s = concat((subject_1.clone(), subject_2.clone(), subject_3.clone()))
		.subscribe(PrintObserver::new("concat_operator"));

	subject_1.next(1);
	subject_1.complete();
	subject_3.next(2); // can't even start as it's subscribed to subject_2 still!
	drop(subject_2); // never completed, so no downstream completion either!
}
