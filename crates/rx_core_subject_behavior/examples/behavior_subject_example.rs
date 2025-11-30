use rx_core::prelude::*;

fn main() {
	let mut subject = BehaviorSubject::<i32>::new(10);
	let mut context = ();

	// Immediately prints "hello 10"
	let mut hello_subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("hello"), &mut context);

	subject.next(11, &mut context);

	let _s1 = subject
		.clone()
		.map(|next| next * 2)
		.subscribe(PrintObserver::<i32>::new("hi double"), &mut context);

	subject.next(12, &mut context);
	hello_subscription.unsubscribe(&mut context);
	subject.next(13, &mut context);
}
