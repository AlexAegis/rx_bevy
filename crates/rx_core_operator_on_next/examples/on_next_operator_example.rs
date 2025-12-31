use rx_core::prelude::*;

/// The [OnNextOperator] calls your fn on subscription
fn main() {
	let _s = (1..=5)
		.into_observable()
		.on_next(|next, destination| destination.next(next * 99))
		.subscribe(PrintObserver::new("on_next_operator"));
}
