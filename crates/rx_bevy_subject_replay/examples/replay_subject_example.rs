use rx_bevy::prelude::*;
use rx_bevy_subject_replay::ReplaySubject;

fn main() {
	let mut subject = ReplaySubject::<i32, 2>::new();

	// Doesn't print out anything on subscribe
	subject.subscribe(PrintObserver::<i32>::new("hello"));

	subject.on_push(1);
	subject.on_push(2);
	subject.on_push(3);

	// Only the last two value is printed out, since our capacity is just 2
	subject.subscribe(PrintObserver::<i32>::new("hi"));

	subject.on_push(4);
	subject.on_push(5);
}
