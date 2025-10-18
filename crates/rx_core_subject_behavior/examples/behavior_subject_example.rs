use rx_core::prelude::*;
use rx_core_subject_behavior::BehaviorSubject;

fn main() {
	let mut subject = BehaviorSubject::<i32>::new(10);

	// Immediately prints "hello 10"
	let mut hello_subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("hello"), &mut ());

	subject.next(12, &mut ());

	let _s1 = subject
		.clone() // Clone since piping needs an owned value, it's still a shared reference over the same set of subscribers
		.map(|next| next * 2)
		.subscribe(PrintObserver::<i32>::new("hi double"), &mut ());

	subject.next(43, &mut ());
	hello_subscription.unsubscribe(&mut ());
	subject.next(11, &mut ());
}
