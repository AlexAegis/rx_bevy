use rx_bevy::prelude::*;

/// [FilterOperator] is used to only let through incoming values that are
/// matching the predicate
fn main() {
	IteratorObservable::new(1..10)
		.filter(|i| i > &10)
		.subscribe(PrintObserver::new("filter_operator"));
}
