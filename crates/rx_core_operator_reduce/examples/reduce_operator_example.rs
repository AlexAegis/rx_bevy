use rx_core::prelude::*;

/// The [ReduceOperator] applies the reducer fn for each upstream emission
/// and stores the result. This result is then only emitted downstream once
/// upstream has completed.
///
/// If you want an downstream emission for every upstream emission, take a look
/// at the [ScanOperator].
fn main() {
	let _s = (0..=10)
		.into_observable::<()>()
		.reduce(|acc, next| acc + next, 0)
		.subscribe(PrintObserver::new("reduce_operator"), &mut ());
}
