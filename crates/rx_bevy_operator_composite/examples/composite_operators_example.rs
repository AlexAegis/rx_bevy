use rx_bevy::prelude::*;

/// Composite operators offer an easy way to create complex operators, but they
/// do increase type complexity, good for prototyping and smaller things, but
/// you should prefer implementing an actual operator
fn main() {
	// Though not necessary, the IdentityOperator provides an easy way to define
	// input types for our composite operator.
	let op = IdentityOperator::<i32, (), ()>::default()
		.pipe(map(|next: i32| next + 1))
		.pipe(map(|next: i32| next * 100));

	let _s = of(1)
		.pipe(op)
		.subscribe(PrintObserver::new("hello"), &mut ());

	// Or though the type extensions you can chain built in operators just like on observables
	let op_2 = IdentityOperator::<i32, (), ()>::default()
		.map(|i| i * 2)
		.filter(|i| i % 2 == 0);

	let _s2 = of(1)
		.pipe(op_2)
		.subscribe(PrintObserver::new("bello"), &mut ());
}
