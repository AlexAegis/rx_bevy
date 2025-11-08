use rx_core::prelude::*;

/// The [CombineLatestObserver] combines the latest values from multiple other
/// observables.
fn main() {
	let mut subject_1 = Subject::<i32>::default();
	let mut subject_2 = Subject::<i32>::default();

	let mut subscription = combine_latest(subject_1.clone(), subject_2.clone())
		.subscribe(PrintObserver::new("combine_latest"), &mut ());

	subject_1.next(1, &mut ());
	subject_2.next(10, &mut ());
	subject_2.next(20, &mut ());

	subject_1.next(2, &mut ());
	subject_1.next(3, &mut ());

	subject_2.next(30, &mut ());

	subject_1.complete(&mut ()); // The first completion won't complete the entire thing
	println!("subject 1 was completed!");
	subject_2.complete(&mut ());
	subscription.unsubscribe(&mut ());
}
