use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.is_empty()
		.subscribe(PrintObserver::new("is_empty_operator - iterator"));

	let _s = empty()
		.is_empty()
		.subscribe(PrintObserver::new("is_empty_operator - empty"));
}
