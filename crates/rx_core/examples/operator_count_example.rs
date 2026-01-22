use rx_core::prelude::*;

/// The [CountOperator] counts upstream emissions and emits the total once the
/// upstream completes.
fn main() {
	let _s = (1..=6)
		.into_observable()
		.filter(|value, _index| value % 2 == 0)
		.count()
		.subscribe(PrintObserver::new("count_operator"));
}
