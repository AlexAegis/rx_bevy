use rx_bevy::prelude::*;

/// The [MulticastOperator] is used to send values to multiple subscribers.
fn main() {
	let mut source = Subject::<i32>::default();

	let mut multicast_subscription = source
		// .multicast()
		.subscribe(PrintObserver::new("multicast_operator"));

	multicast_subscription.unsubscribe();
}
