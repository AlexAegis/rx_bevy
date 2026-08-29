use rx_core::prelude::*;

/// The throw observer immediately emits an error upon subscription
///
/// Output:
///
/// ```sh
/// throw_example - error: "hello"
/// throw_example - unsubscribed
/// ```
fn main() {
	let _s = throw("hello").subscribe(PrintObserver::new("throw_example"));
}
