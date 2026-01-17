use rx_core::prelude::*;

/// The [PrintObserver] prints incoming next, error and complete notifications
/// with the supplied message prepended to it
fn main() {
	let _s = just(1).subscribe(PrintObserver::new("hello"));
}
