use rx_core::prelude::*;

/// [FilterOperator] is used to only let through incoming values that are
/// matching the predicate
fn main() {
	let _s = (1..=5)
		.into_observable::<()>()
		.pipe(map(|next: i32| next + 1))
		.filter(|i| i > &2)
		.subscribe(PrintObserver::new("filter_operator"), &mut ());
}
