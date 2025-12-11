use rx_core::prelude::*;

fn main() {
	let mut first_subject = Subject::<i32>::default();
	let second_subject = Subject::<i32>::default();

	let _second_subject_subscription = second_subject
		.clone()
		.finalize(|| println!("finalize 1"))
		.subscribe(PrintObserver::<i32>::new("second_subject"));

	first_subject.next(1);

	let mut first_subject_subscription_1 = first_subject
		.clone()
		.tap_next(|next| println!("first_subject sub 1 {}", next))
		.finalize(|| println!("finalize 0"))
		.subscribe(second_subject.clone());

	first_subject.next(2);
	first_subject.next(3);

	first_subject_subscription_1.unsubscribe();

	// TODO: Bug, this should not be logged by the tap of the first, just unsubscribed subscription!
	first_subject.next(4);

	let mut _first_subject_subscription_2 = first_subject
		.clone()
		.tap_next(|next| println!("first_subject sub 2 {}", next))
		.finalize(|| println!("finalize 0"))
		.subscribe(second_subject.clone());

	first_subject.next(5);
}
