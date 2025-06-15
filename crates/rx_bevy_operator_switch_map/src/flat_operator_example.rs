use rx_bevy::prelude::*;

fn main() {
	of(1)
		.map(|next| of(next * 2))
		.flat(SwitchFlattener::default())
		.subscribe(PrintObserver::new("lifted, then flattened value"));
}
