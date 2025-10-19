use rx_core::prelude::*;

/// The throw observer immediately emits an error upon subscription
///
/// Output:
///
/// ```sh
/// throw_example - error: "hello"
/// ```
fn main() {
	let _s = throw::<_, ()>("hello").subscribe(PrintObserver::new("throw_example"), &mut ());
}
