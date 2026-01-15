use rx_core::prelude::*;

fn main() {
	let composite_operator = compose_operator::<i32, Never>()
		.map(|i| i + 1)
		.filter(|i, _| i < &4);

	let _s = (1..=5)
		.into_observable()
		.pipe(composite_operator)
		.subscribe(PrintObserver::new("identity_operator (composite)"));
}
