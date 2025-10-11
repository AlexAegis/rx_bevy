use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	let context = &mut ();

	let mut subscription_1 = subject
		.clone()
		.finalize(|_| println!("finalize 0"))
		.subscribe(
			PrintObserver::<i32>::new("subject_example (subscription 0)"),
			context,
		);

	subject.next(1, context);

	// Bind subscriptions to a variable if you want it to live until the end of the block (naming it "_" doesn't do that)
	let _subscription_2 = subject
		.clone()
		.finalize(|_| println!("finalize 1"))
		.subscribe(
			PrintObserver::<i32>::new("subject_example (subscription 1)"),
			context,
		);

	subject.next(2, context);
	subscription_1.unsubscribe(context);
	subject.next(3, context);
	subject.complete(context);
	subject.next(4, context); // Won't get emitted as it's already closed
}
