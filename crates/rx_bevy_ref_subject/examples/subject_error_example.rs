use rx_bevy::prelude::*;
use rx_bevy_ref_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	let mut subscription_1 = subject
		.clone()
		.finalize(|_| println!("finalize 0"))
		.subscribe(
			PrintObserver::<i32>::new("subject_example (subscription 0)"),
			&mut (),
		);

	subject.next(1, &mut ());

	// Bind subscriptions to a variable if you want it to live until the end of the block (naming it "_" doesn't do that)
	let _subscription_2 = subject
		.clone()
		.finalize(|_| println!("finalize 0"))
		.subscribe(
			PrintObserver::<i32>::new("subject_example (subscription 1)"),
			&mut (),
		);

	subject.next(2, &mut ());
	subscription_1.unsubscribe(&mut ());
	subject.next(3, &mut ());
	subject.complete(&mut ());
}
