use rx_core::prelude::*;

/// The [SkipOperator] is used to skip the first `n` emissions of an observable,
/// letting everything else through after.
fn main() {
	let _s = (1..=5)
		.into_observable::<()>()
		.skip(2)
		.subscribe(PrintObserver::new("skip_operator"), &mut ());
}
