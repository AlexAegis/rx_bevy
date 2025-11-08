use rx_core::prelude::*;

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
	let mut context = ();
	let mut subject = Subject::<i32>::default();
	let mut subscription = subject
		.clone()
		.finalize(|_| println!("finally!"))
		.subscribe(PrintObserver::new("finalize_operator"), &mut context);

	subject.next(1, &mut context);
	subject.next(2, &mut context);
	subscription.unsubscribe(&mut context);
}
