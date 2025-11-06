use rx_core::prelude::*;

fn main() {
	let context = &mut ();
	let mut first_subject = Subject::<i32>::default();
	let second_subject = Subject::<i32>::default();

	let _second_subject_subscription = second_subject
		.clone()
		.finalize(|_| println!("finalize 1"))
		.subscribe(PrintObserver::<i32>::new("second_subject"), context);

	first_subject.next(1, context);

	let mut first_subject_subscription_1 = first_subject
		.clone()
		.tap_next(|next, _| println!("first_subject sub 1 {}", next))
		.finalize(|_| println!("finalize 0"))
		.subscribe(second_subject.clone(), context);

	first_subject.next(2, context);
	first_subject.next(3, context);

	first_subject_subscription_1.unsubscribe(context);

	// TODO: Bug, this should not be logged by the tap of the first, just unsubscribed subscription!
	first_subject.next(4, context);

	let mut _first_subject_subscription_2 = first_subject
		.clone()
		.tap_next(|next, _| println!("first_subject sub 2 {}", next))
		.finalize(|_| println!("finalize 0"))
		.subscribe(second_subject.clone(), context);

	first_subject.next(5, context);
}
