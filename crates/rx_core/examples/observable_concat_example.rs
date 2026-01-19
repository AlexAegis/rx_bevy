use rx_core::prelude::*;

use rx_core_observable_concat::observable_fn::concat;

fn main() {
	let mut subject_1 = PublishSubject::<usize>::default();
	let mut subject_2 = PublishSubject::<usize>::default();
	let mut subject_3 = PublishSubject::<usize>::default();

	let _s = concat((
		subject_1.clone(),
		subject_2.clone().take(2),
		subject_3.clone(),
	))
	.subscribe(PrintObserver::new("concat_operator"));

	subject_1.next(1);
	subject_1.complete();
	subject_3.complete();
	subject_2.next(2);
	subject_2.next(3); // also completes because of take
}
