use rx_core::prelude::*;

fn main() {
	let mut subject = PublishSubject::<usize>::default();
	let mut subscription = subject
		.clone()
		.take(1)
		.subscribe(PrintObserver::new("subject"));
	let mut subject_teardown = subject.clone();
	subscription.add_fn(move || {
		println!("teardown");
		subject_teardown.unsubscribe();
	});
	println!("nexting one");
	subject.next(1);
	println!("nexted one");

	subscription.unsubscribe();
	println!("end");
}
