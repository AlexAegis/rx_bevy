use rx_core::prelude::*;

fn main() {
	let mut subject = Subject::<i32>::default();
	subject.next(1); // Meteora - Track 11

	let mut subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("subject_example"));

	subject.next(2);
	subject.next(3);
	subscription.unsubscribe();
	subject.next(4);
}
