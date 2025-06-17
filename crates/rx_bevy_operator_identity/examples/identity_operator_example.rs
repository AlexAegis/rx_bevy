use rx_bevy::prelude::*;

/// The [IdentityOperator] does nothing. The only purpose it has
/// is to define inputs for a [CompositeOperator]: an [Operator] that made out
/// of other [Operator]s without having to use a [Pipe] which would require a
/// source [Observable]
fn main() {
	of(12)
		.identity()
		.subscribe(PrintObserver::new("identity_operator (useless)"));

	let composite_operator = IdentityOperator::<i32, ()>::default()
		.map(|i| i + 1)
		.filter(|i| i < &4);

	IteratorObservable::new(1..=10)
		.pipe(composite_operator)
		.subscribe(PrintObserver::new("identity_operator (composite)"));
}
