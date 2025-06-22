use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	let mut subscription_1 = subject
		.clone()
		.finalize(|| println!("finalize 0"))
		.subscribe(PrintObserver::<i32>::new(
			"subject_example (subscription 0)",
		));

	subject.next(1);

	// Bind subscriptions to a variable if you want it to live until the end of the block (naming it "_" doesn't do that)
	let _subscription_2 = subject
		.clone()
		.finalize(|| println!("finalize 0"))
		.subscribe(PrintObserver::<i32>::new(
			"subject_example (subscription 1)",
		));

	subject.next(2);
	subscription_1.unsubscribe();
	subject.next(3);
	subject.complete();
}
