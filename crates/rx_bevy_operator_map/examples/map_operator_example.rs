use rx_bevy::prelude::*;

/// The [MapOperator] is used to transform incoming values into something else
fn main() {
	IteratorObservable::new(1..=10)
		.map(|i| i * 2)
		.map(|i| (format!("the double of {i} is: "), i * 2))
		.subscribe(PrintObserver::new("map_operator"));
}
