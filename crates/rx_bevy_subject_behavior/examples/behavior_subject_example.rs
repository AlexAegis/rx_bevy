use rx_bevy::prelude::*;
use rx_bevy_subject_behavior::BehaviorSubject;

fn main() {
	let mut subject = BehaviorSubject::<i32>::new(10);

	// Immediately prints "hello 10"
	let mut hello_subscription = subject.subscribe(PrintObserver::<i32>::new("hello"));

	subject.on_push(12);

	subject
		.clone() // Clone since piping needs an owned value, it's still a shared reference over the same set of subscribers
		.map(|next| next * 2)
		.subscribe(PrintObserver::<i32>::new("hi double"));

	subject.on_push(43);
	hello_subscription.unsubscribe();
	subject.on_push(11);
}
