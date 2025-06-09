use rx_bevy::prelude::*;

fn main() {
	of(of(1))
		.flat()
		.subscribe(PrintObserver::new("flattened value"));
}
