use rx_core::prelude::*;

fn main() {
	let mut subject = BehaviorSubject::<i32>::new(10);

	// Immediately prints "hello 10"
	let mut hello_subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("hello"));

	subject.next(11);

	let _s1 = subject
		.clone()
		.map(|next| next * 2)
		.subscribe(PrintObserver::<i32>::new("hi double"));

	subject.next(12);
	hello_subscription.unsubscribe();
	subject.next(13);
	subject.complete();

	let mut _compelted_subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("hello_completed"));
}
