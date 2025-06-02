use rx_bevy::prelude::*;
use rx_bevy_observable_throw::throw;

/// The throw observer immediately emits an error upon subscription
///
/// Output:
///
/// ```sh
/// throw_example - error: "hello"
/// ```
fn main() {
	throw("hello").subscribe(PrintObserver::new("throw_example"));
}
