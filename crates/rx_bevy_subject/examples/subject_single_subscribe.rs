use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	println!("example 1");

	let mut _subscription_1 = subject.clone().subscribe(PrintObserver::<i32>::new(
		"subject_example (subscription 1)",
	));

	println!("example 3");

	subject.next(12);
	subject.next(43);
	println!("example 4");
	// _subscription_1.unsubscribe();
	println!("example 5");
	subject.next(11);
	subject.complete();
	println!("example end");
}
