use rx_core::prelude::*;

/// The [FilterMapOperator] is used to transform incoming values into an [Option]
/// of something else, combining the [FilterOperator] and [MapOperator]
fn main() {
	let _s = (1..=5)
		.into_observable()
		.filter_map(|i| if i % 2 == 0 { Some(i) } else { None })
		.subscribe(PrintObserver::new("filter_map_operator"));
}
