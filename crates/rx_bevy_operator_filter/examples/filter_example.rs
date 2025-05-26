use rx_bevy::prelude::*;
use rx_bevy_operator_filter::prelude::ObservableExtensionFilter;

/// The map operator is used to transform incoming values into something else
fn main() {
	of(1)
		.filter(|i| i > &10)
		.subscribe(PrintObserver::new("filtered:"));
}
