use rx_bevy::prelude::*;
use rx_bevy_ref_subject_replay::ReplaySubject;

fn main() {
	let mut subject = ReplaySubject::<2, i32>::default();

	// Doesn't print out anything on subscribe
	let _s = subject
		.clone()
		.subscribe(PrintObserver::<i32>::new("hello"));

	subject.next(1);
	subject.next(2);
	subject.next(3);

	// Only the last two value is printed out, since our capacity is just 2
	let _s2 = subject.clone().subscribe(PrintObserver::<i32>::new("hi"));

	subject.next(4);
	subject.next(5);
}
