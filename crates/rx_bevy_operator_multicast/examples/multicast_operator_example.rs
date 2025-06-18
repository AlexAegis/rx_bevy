use rx_bevy::prelude::*;

/// The [MulticastOperator] is used to send values to multiple subscribers.
fn main() {
	let mut multicast_subscription = (1..=5)
		.into_observable()
		.multicast()
		.subscribe(PrintObserver::new("multicast_operator"));

	multicast_subscription.unsubscribe();
}
