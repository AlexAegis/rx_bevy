use rx_core::prelude::*;

fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| if i % 2 == 0 { Some(i) } else { None })
		.lift_option()
		.subscribe(PrintObserver::new("lift_option_operator"));
}
