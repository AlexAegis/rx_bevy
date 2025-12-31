use rx_core::prelude::*;

/// The [MapErrorOperator] is used to transform incoming values into something else,
/// in this case into a tuple of a string and number!
fn main() {
	let _s = concat((
		(1..=5)
			.into_observable()
			.map_error(Never::map_into::<&'static str>()),
		throw("error").map(Never::map_into::<usize>()),
	))
	.skip(1)
	.map_error(|error| format!("error? {error}"))
	.subscribe(PrintObserver::new("map_error_operator"));
}
