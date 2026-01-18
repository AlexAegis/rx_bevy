use rx_core::prelude::*;

fn main() {
	let _s = concat((
		(1..=3).into_observable().map_never(),
		throw("error").map_never(),
	))
	.map(|i| i * 10)
	.catch(|_error| IteratorObservable::new(90..=92))
	.subscribe(PrintObserver::new("catch"));
}
