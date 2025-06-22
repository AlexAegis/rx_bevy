use rx_bevy::prelude::*;

/// The [FinalizeOperator]s closure will be called upon **either** when the
/// source [Observable] completes, or when the subscription gets unsubscribed.
///
/// It will only be called once per subscription and consumed!
///
/// Output:
///
/// ```sh
/// finalize_example - next: 1
/// finalize_example - next: 2
/// finally!
/// ```
///
/// > Notice how there is no completion signal, the source didn't complete, we
/// > just stopped listening.
///
fn main() {
	let mut subject = Subject::<i32>::default();
	let mut subscription = subject
		.clone()
		.finalize(|| println!("finally!"))
		.subscribe(PrintObserver::new("finalize_operator"));

	subject.next(1);
	subject.next(2);
	subscription.unsubscribe();
}
