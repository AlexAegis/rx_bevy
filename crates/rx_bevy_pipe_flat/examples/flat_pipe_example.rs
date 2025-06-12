use rx_bevy::prelude::*;

fn main() {
	of(of(1))
		.flat(SwitchFlattener::new())
		.subscribe(PrintObserver::new("flattened value"));
}
