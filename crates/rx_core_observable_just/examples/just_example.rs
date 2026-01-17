use rx_core::prelude::*;

fn main() {
	let _s = just("hello").subscribe(NoopObserver::default());
}
