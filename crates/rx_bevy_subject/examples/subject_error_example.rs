use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32, String>::new();

	let mut hello_subscription = subject.subscribe(PrintObserver::<i32, String>::new("hello"));
	subject.subscribe(PrintObserver::<i32, String>::new("hi"));
	println!(
		"1 hello_subscription is_closed {}",
		hello_subscription.is_closed()
	);
	subject.on_push(12);
	subject.on_error("ERROR".to_string());
	subject.on_error("SECOND ERROR".to_string()); // Should not be printed
	println!(
		"2 hello_subscription is_closed {}",
		hello_subscription.is_closed()
	);
	hello_subscription.unsubscribe();
}
