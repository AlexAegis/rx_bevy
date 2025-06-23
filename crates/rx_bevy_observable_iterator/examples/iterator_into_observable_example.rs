use rx_bevy::prelude::*;

fn main() {
	let _r_s = (1..=3)
		.into_observable()
		.subscribe(PrintObserver::new("range"));

	let _v_s = vec![1, 2, 3]
		.into_observable()
		.subscribe(PrintObserver::new("vector"));

	let _a_s = [1, 2, 3]
		.into_observable()
		.subscribe(PrintObserver::new("array"));
}
