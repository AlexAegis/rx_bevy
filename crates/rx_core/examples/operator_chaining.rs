use rx_core::prelude::*;

/// This is how you'd normally use operators that are extending Observable
/// to provide simple methods to create them
fn main() {
	let _s = of(12)
		.map(|n| n * 2)
		.map(|n| n.to_string())
		.subscribe(PrintObserver::new("hello"));
}
