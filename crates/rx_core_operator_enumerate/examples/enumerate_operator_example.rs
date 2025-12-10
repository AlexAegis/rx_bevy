use rx_core::prelude::*;

/// The [EnumerateOperator] counts emissions, and downstream receives this
/// counter in a tuple with the emitted value as (T, usize)
fn main() {
	let _s = (10..=15)
		.into_observable()
		.enumerate()
		.subscribe(PrintObserver::new("enumerate_operator"));
}
