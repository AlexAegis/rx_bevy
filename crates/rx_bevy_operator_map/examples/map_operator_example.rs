use rx_bevy::prelude::*;

/// The [MapOperator] is used to transform incoming values into something else,
/// in this case into a tuple of a string and number!
fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| (format!("the double of {i} is: "), i * 2))
		.subscribe(PrintObserver::new("map_operator"), &mut ());
}
