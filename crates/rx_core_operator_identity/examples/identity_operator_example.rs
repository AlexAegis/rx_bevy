use rx_core::prelude::*;

fn main() {
	let _s = IdentityOperator::default()
		.operate(of(1))
		.subscribe(PrintObserver::new("identity_operator"));
}
