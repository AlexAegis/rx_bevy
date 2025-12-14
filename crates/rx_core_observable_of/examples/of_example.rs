use rx_core::prelude::*;

fn main() {
	let _s = of("hello").subscribe(NoopObserver::default());
}
