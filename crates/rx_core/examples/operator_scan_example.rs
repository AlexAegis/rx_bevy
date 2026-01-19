use rx_core::prelude::*;

/// The [ScanOperator] applies the reducer fn for each upstream emission
/// and stores the result. This result is then immediately emitted downstream.
///
/// If you don't want a downstream emission for every upstream emission, and
/// instead you want to receive a single value on completion, use the
/// [ReduceOperator].
fn main() {
	let _s = (0..=10)
		.into_observable()
		.scan(|acc, next| acc + next, 0)
		.subscribe(PrintObserver::new("scan_operator"));
}
