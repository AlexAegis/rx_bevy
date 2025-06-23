use rx_bevy::prelude::*;

/// The map operator is used to transform incoming values into something else
fn main() {
	let _s = of(1)
		.map(|i| i + 1)
		.subscribe(PrintObserver::new("mapped:"));
}
