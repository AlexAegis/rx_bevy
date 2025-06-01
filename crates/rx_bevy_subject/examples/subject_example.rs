use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::new();

	let mut hello_subscription = subject.subscribe(PrintObserver::<i32>::new("hello"));
	// subject
	// 	.map(|next| next * 2)
	// 	.subscribe(PrintObserver::<i32>::new("hi double"));

	subject.on_push(12);
	subject.on_push(43);
	hello_subscription.unsubscribe();
	subject.on_push(11);
}
