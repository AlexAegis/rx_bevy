use rx_core::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let _s = empty().subscribe(PrintObserver::default());
}
