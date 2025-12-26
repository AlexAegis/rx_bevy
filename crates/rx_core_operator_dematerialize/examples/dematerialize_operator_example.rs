use rx_core::prelude::*;

fn main() {
	let _s = [
		ObserverNotification::<_, Never>::Next(1),
		ObserverNotification::Complete,
	]
	.into_observable()
	.dematerialize()
	.subscribe(PrintObserver::new("dematerialize_operator"));
}
