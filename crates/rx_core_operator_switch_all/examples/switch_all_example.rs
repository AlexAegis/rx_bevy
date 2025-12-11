use rx_core::prelude::*;

fn main() {
	let mut subscription = (1..=3)
		.into_observable()
		.finalize(|| println!("finalize: upstream"))
		.tap_next(|n| println!("emit (source): {n}"))
		.map(|next| {
			(next..=3)
				.into_observable()
				.map(move |i| format!("from {next} through 3, current: {i}"))
				.finalize(|| println!("finalize: inner"))
				.tap_next(|n| println!("emit (inner): '{n}'"))
		})
		.switch_all()
		.finalize(|| println!("finalize: downstream"))
		.subscribe(PrintObserver::new("switch_map"));
	subscription.unsubscribe();
}
