use rx_bevy::{PrintObserver, of};
use rx_bevy_observable::prelude::*;

use rx_bevy_operator_filter_map::prelude::ObservableExtensionMap;

/// The map operator is used to transform incoming values into something else
fn main() {
	of(12)
		.filter_map(|i| if i % 2 == 0 { Some(i) } else { None })
		.subscribe(PrintObserver::new("hello"))
}
