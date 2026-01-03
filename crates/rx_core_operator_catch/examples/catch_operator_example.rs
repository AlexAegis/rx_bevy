use rx_core::prelude::*;

fn main() {
	let _s = concat((
		(1..=3)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		throw("error").map(Never::map_into::<i32>()),
	))
	.map(|i| i * 10)
	.catch(|_error| IteratorObservable::new(90..=92))
	.subscribe(PrintObserver::new("catch"));
}
