use rx_bevy::prelude::*;

/// The [TapOperator] is used to peek inside a stream without changing its behavior
fn main() {
	let _s = (1..=5)
		.into_observable()
		.tap_next(|next, _context| println!("hello {next}"))
		.subscribe(PrintObserver::new("tap_operator"), &mut ());
}
