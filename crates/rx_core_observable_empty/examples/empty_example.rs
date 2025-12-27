use rx_core::prelude::*;

fn main() {
	let _s = empty().subscribe(PrintObserver::new("empty"));
}
