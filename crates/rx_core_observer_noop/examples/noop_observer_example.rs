use rx_core::prelude::*;

/// The [NoopObserver] does nothing with the received values
fn main() {
	let _s = just(1).subscribe(NoopObserver::default());
}
