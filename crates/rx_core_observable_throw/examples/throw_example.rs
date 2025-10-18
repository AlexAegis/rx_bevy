use rx_core::prelude::*;
use rx_core_observable_throw::throw;

/// The throw observer immediately emits an error upon subscription
///
/// Output:
///
/// ```sh
/// throw_example - error: "hello"
/// ```
fn main() {
	let _s = throw("hello").subscribe(PrintObserver::new("throw_example"), &mut ());
}
