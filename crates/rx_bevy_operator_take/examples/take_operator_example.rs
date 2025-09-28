use rx_bevy::prelude::*;

/// The [TakeOperator] is used to limit how much events can be observed before
/// a forced completion
fn main() {
	let _s = (1..=5)
		.into_observable()
		.take(2)
		.subscribe(PrintObserver::new("take_operator"), &mut ());
}
