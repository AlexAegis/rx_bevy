use rx_core::prelude::*;

fn main() {
	let mut subject = Subject::<i32>::default();
	let mut context = ();

	subject.next(1, &mut context);

	let mut subscription_1 = subject
		.clone()
		.finalize(|_| println!("finalize subscription 1"))
		.subscribe(
			PrintObserver::<i32>::new("subject_subscription_1"),
			&mut context,
		);

	subject.next(2, &mut context);

	let _subscription_2 = subject
		.clone()
		.finalize(|_| println!("finalize subscription 2"))
		.subscribe(
			PrintObserver::<i32>::new("subject_subscription_2"),
			&mut context,
		);

	subject.next(3, &mut context);

	subscription_1.unsubscribe(&mut context);

	subject.next(4, &mut context);
}
