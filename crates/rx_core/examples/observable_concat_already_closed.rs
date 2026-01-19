use rx_core::prelude::*;

use rx_core_observable_concat::observable_fn::concat;

fn main() {
	let mut subject_1 = PublishSubject::<usize>::default();
	subject_1.complete();

	let _s = concat((subject_1.clone(),)).subscribe(PrintObserver::new("concat_operator"));
}
