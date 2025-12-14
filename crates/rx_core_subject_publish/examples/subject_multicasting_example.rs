use rx_core::prelude::*;

fn main() {
	let mut subject = PublishSubject::<i32>::default();

	subject.next(1);

	let mut subscription_1 = subject
		.clone()
		.finalize(|| println!("finalize subscription 1"))
		.subscribe(PrintObserver::<i32>::new("subject_subscription_1"));

	subject.next(2);

	let _subscription_2 = subject
		.clone()
		.finalize(|| println!("finalize subscription 2"))
		.subscribe(PrintObserver::<i32>::new("subject_subscription_2"));

	subject.next(3);

	subscription_1.unsubscribe();

	subject.next(4);
}
