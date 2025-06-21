use rx_bevy::prelude::*;

/// The [MulticastOperator] is used to send values to multiple subscribers.
fn main() {
	let mut multicaster = (1..=2).into_observable();

	// let mu = multicaster.subscribe(PrintObserver::new("multicast_operator"));

	// multicaster.unsubscribe();
}
