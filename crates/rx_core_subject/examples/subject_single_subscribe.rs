use rx_core::prelude::*;
use rx_core_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	println!("example 1");

	let mut _subscription_1 = subject.clone().subscribe(
		PrintObserver::<i32>::new("subject_example (subscription 1)"),
		&mut (),
	);

	println!("example 3");

	subject.next(12, &mut ());
	subject.next(43, &mut ());
	println!("example 4");
	// _subscription_1.unsubscribe(&mut ());
	println!("example 5");
	subject.next(11, &mut ());
	subject.complete(&mut ());
	println!("example end");
}
