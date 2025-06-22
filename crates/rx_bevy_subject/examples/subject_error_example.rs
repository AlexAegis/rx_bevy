use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32, String>::default();

	let mut hello_subscription = subject
		.clone()
		.subscribe(PrintObserver::<i32, String>::new("hello"));
	subject
		.clone()
		.subscribe(PrintObserver::<i32, String>::new("hi"));
	println!(
		"1 hello_subscription is_closed {}",
		hello_subscription.is_closed()
	);
	subject.next(12);
	subject.error("ERROR".to_string());
	subject.error("SECOND ERROR".to_string()); // Should not be printed
	println!(
		"2 hello_subscription is_closed {}",
		hello_subscription.is_closed()
	);
	hello_subscription.unsubscribe();
}
