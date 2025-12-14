use rx_core::prelude::*;

use rx_core_observable_concat::observable_fn::concat;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let mut subject_1 = Subject::<usize>::default();
	let mut subject_2 = Subject::<usize>::default();
	let subject_3 = Subject::<usize>::default();

	let _s = concat((
		subject_1.clone(),
		subject_3.clone(),
		subject_2.clone().take(2),
	))
	.subscribe(PrintObserver::new("concat_operator"));

	subject_1.next(1);
	subject_1.complete();
	subject_2.next(2);
	subject_2.next(3); // also completes because of take
	println!("asd2");
	drop(subject_3); // never completed, so no downstream completion either!
	println!("asd23");
	drop(_s);
	println!("asd")
}
