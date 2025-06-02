use rx_bevy::prelude::*;

/// The finalize operators closure will only be called once per subscription!
///
/// Output:
///
/// ```sh
/// finalize_example - next: 12
/// finally!
/// finalize_example - completed
/// ```
fn main() {
	of(12)
		.finalize(|| println!("finally!"))
		.subscribe(PrintObserver::new("finalize_example"));
}
