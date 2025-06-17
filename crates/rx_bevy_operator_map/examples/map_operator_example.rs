use rx_bevy::prelude::*;

/// The [MapOperator] is used to transform incoming values into something else
fn main() {
	(1..=5)
		.into_observable()
		.map(|i| (format!("the double of {i} is: "), i * 2))
		.subscribe(PrintObserver::new("map_operator"));
}
