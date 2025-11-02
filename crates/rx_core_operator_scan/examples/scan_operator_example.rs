use rx_core::prelude::*;

/// The [ScanOperator] applies the reducer clojure for each upstream emission
/// and stores the result. This result is then immediately emitted downstream.
fn main() {
	let _s = (0..=10)
		.into_observable::<()>()
		.scan(|acc, next| acc + next, 0)
		.subscribe(PrintObserver::new("scan_operator"), &mut ());
}
