use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	of(12)
		.identity()
		.subscribe(PrintObserver::new("nothing changed"));
}
