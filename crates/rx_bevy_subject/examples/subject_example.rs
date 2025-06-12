use rx_bevy::prelude::*;
use rx_bevy_subject::Subject;

fn main() {
	let mut subject = Subject::<i32>::default();

	let mut subscription_1 = subject.subscribe(PrintObserver::<i32>::new(
		"subject_example (subscription 1)",
	));
	subject
		.clone() // Clone since piping needs an owned value, it's still a shared reference over the same set of subscribers
		.map(|next| next * 2)
		.subscribe(PrintObserver::<i32>::new(
			"subject_example (subscription 2)",
		));

	subject.next(12);
	subject.next(43);
	subscription_1.unsubscribe();
	subject.next(11);
	subject.complete();
}
