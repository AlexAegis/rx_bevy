use rx_bevy::prelude::*;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// after which it does nothing.
fn main() {
	let _s = (1..=5)
		.into_observable()
		.skip(2)
		.subscribe(PrintObserver::new("skip_operator"));
}
