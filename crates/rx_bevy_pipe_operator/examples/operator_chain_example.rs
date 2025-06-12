use rx_bevy::prelude::*;

/// The map operator is used to transform incoming values into something else
fn main() {
	let _a = of(20);

	// let a = MapOperator::new(|i: i32| i + 1);

	// OperatorChain::Root(IdentityOperator::default());

	//of(1)
	//	.pipe(OperatorPipe::new(MapOperator::new(|i| i * 2)))
	//	.map(|i| i + 1)
	//	.subscribe(PrintObserver::new("mapped:"));
}
