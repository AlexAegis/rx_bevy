use rx_bevy::prelude::*;

/// The [IdentityOperator] does nothing. The only purpose it has
/// is to define inputs for a [CompositeOperator]: an [Operator] that made out
/// of other [Operator]s without having to use a [Pipe] which would require a
/// source [Observable]
fn main() {
	let composite_operator = IdentityOperator::<i32, (), ()>::default()
		.map(|i| i + 1)
		.filter(|i| i < &4);

	let _s = (1..=5)
		.into_observable::<()>()
		.pipe(composite_operator)
		.subscribe(PrintObserver::new("identity_operator (composite)"), &mut ());
}
