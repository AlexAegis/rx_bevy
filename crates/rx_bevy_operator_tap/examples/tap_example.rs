use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	of(12)
		.tap_next(|next| println!("hello {next}"))
		.subscribe(NoopObserver::default());
}
