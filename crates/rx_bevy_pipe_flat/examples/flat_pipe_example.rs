use rx_bevy::prelude::*;

fn main() {
	of(of(1))
		.flat(SwitchForwarder::new())
		.subscribe(PrintObserver::new("flattened value"));
}
