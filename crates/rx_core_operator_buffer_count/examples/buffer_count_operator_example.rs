use rx_core::prelude::*;

fn main() {
	// In case you don't like curved magazines
	let _s = (1..=25)
		.into_observable()
		.buffer_count(3)
		.subscribe(PrintObserver::new("buffer_count_operator"));
}
