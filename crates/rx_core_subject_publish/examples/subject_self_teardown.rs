use rx_core::prelude::*;

fn main() {
	let mut subject = PublishSubject::<usize>::default();

	let _s = subject.clone().subscribe(PrintObserver::new("subjectboy"));

	let mut subject_teardown = subject.clone();

	let mut subscription = subject
		.clone()
		.finalize(move || {
			subject_teardown.next(2); // Will be deferred, so this subscription wont get it
			println!("teardown");
			subject_teardown.unsubscribe();
		}) // test with take 0 too, instantly unsubscribing!
		.take(1)
		.subscribe(PrintObserver::new("subject_sub_with_finalize"));

	println!("nexting one");
	subject.next(1);
	println!("nexted one");

	subscription.unsubscribe();
	println!("end");
}
