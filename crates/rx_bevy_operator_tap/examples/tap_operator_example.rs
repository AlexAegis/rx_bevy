use rx_bevy::prelude::*;

/// The [TapOperator] is used to peek inside a stream without changing its behavior
fn main() {
	IteratorObservable::new(1..=5)
		.tap_next(|next| println!("hello {next}"))
		.subscribe(PrintObserver::new("tap_operator"));
}
