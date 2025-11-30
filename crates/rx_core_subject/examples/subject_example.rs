use rx_core::prelude::*;

fn main() {
	let mut subject = Subject::<i32>::default();
	let mut context = ();
	subject.next(1, &mut context); // Meteora - Track 11

	let mut subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("subject_example"), &mut context);

	subject.next(2, &mut context);
	subject.next(3, &mut context);
	subscription.unsubscribe(&mut context);
	subject.next(4, &mut context);
}
