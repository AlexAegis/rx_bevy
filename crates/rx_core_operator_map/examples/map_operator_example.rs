use rx_core::prelude::*;

/// The [MapOperator] is used to transform incoming values into something else,
/// in this case into a tuple of a string and number!
fn main() {
	let _s = (1..=5)
		.into_observable::<()>()
		.map(|i| i * 2)
		.skip(1)
		.subscribe(PrintObserver::new("map_operator"), &mut ());
}
