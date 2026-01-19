use rx_core::prelude::*;

fn main() {
	let mut retried = concat((
		(0..=2)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		throw("error").map(Never::map_into::<usize>()),
	))
	.retry(2);

	let _s1 = retried.subscribe(PrintObserver::new("retry_operator"));
}
