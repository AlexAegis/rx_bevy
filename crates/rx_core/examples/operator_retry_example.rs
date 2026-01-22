use rx_core::prelude::*;

fn main() {
	let mut retried = concat((
		(0..=2).into_observable().map_never(),
		throw("error").map_never(),
	))
	.retry(2);

	let _s1 = retried.subscribe(PrintObserver::new("retry_operator"));
}
