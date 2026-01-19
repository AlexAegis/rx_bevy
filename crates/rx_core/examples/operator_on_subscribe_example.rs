use rx_core::prelude::*;

/// The [OnSubscribeOperator] calls your fn on subscription
fn main() {
	let _s = (1..=5)
		.into_observable()
		.on_subscribe(|destination| destination.next(99))
		.subscribe(PrintObserver::new("on_subscribe_operator"));
}
