use rx_bevy::prelude::*;

fn main() {
	(1..=3)
		.into_observable()
		.subscribe(PrintObserver::new("range"));

	vec![1, 2, 3]
		.into_observable()
		.subscribe(PrintObserver::new("vector"));

	[1, 2, 3]
		.into_observable()
		.subscribe(PrintObserver::new("array"));
}
