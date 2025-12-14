use rx_core::prelude::*;

#[derive(Clone, Debug)]
enum Either {
	Left,
	Right,
	Top,
}

fn main() {
	let mut source = PublishSubject::<Either>::default();
	let mut left = PublishSubject::<i32>::default();
	let mut right = PublishSubject::<i32>::default();
	let mut top = PublishSubject::<i32>::default();
	let left_clone = left.clone();
	let right_clone = right.clone();
	let top_clone = top.clone();
	let mut subscription = source
		.clone()
		.exhaust_map(move |next| match next {
			Either::Left => left_clone.clone(),
			Either::Right => right_clone.clone(),
			Either::Top => top_clone.clone(),
		})
		.subscribe(PrintObserver::new("exhaust_map"));

	source.next(Either::Left);
	left.next(1);
	right.next(-1);
	source.next(Either::Right); // Nothing because the inner one hasn't completed yet!
	left.next(2); // Still goes through as no switch has happened.
	left.unsubscribe(); // Both unsubscribe and complete works to mark the inner observable completed

	source.next(Either::Right); // Successful switch, the previous inner observable completed
	right.next(-2);
	right.complete();

	source.next(Either::Left); // Switching back to an already completed one
	left.next(3); // Nothing happens, it's already closed
	source.next(Either::Top); // Can switch since the last one was already closed
	top.next(10);

	source.unsubscribe();

	println!("end");

	subscription.unsubscribe();
}
