use rx_core::prelude::*;

fn main() {
	let mut context = ();

	let mut subscription = (1..=3)
		.into_observable::<()>()
		.finalize(|_context| println!("finalize: upstream"))
		.tap_next(|n, _context| println!("emit (source): {n}"))
		.map(|next| {
			(next..=3)
				.into_observable()
				.map(move |i| format!("from {next} through 3, current: {i}"))
				.finalize(|_context| println!("finalize: inner"))
				.tap_next(|n, _context| println!("emit (inner): '{n}'"))
		})
		.switch_all()
		.finalize(|_context| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"), &mut context);
	subscription.unsubscribe(&mut context);
}
