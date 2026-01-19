use rx_core::prelude::*;

/// The [IdentityOperator] does nothing. The only purpose it has
/// is to define inputs for a [CompositeOperator]: an [Operator] that made out
/// of other [Operator]s without having to use a [Pipe] which would require a
/// source [Observable]
fn main() {
	let _s = (1..=5)
		.into_observable()
		.map(|i| i * 2)
		.error_boundary()
		.subscribe(PrintObserver::new("error_boundary_operator (composite)"));

	// This cannot compile as relative to the `error_boundary` operator,
	// upstreams error type is not `Never`
	// let _s2 = throw("error".to_string())
	// 	.map(|i| i)
	// 	.error_boundary()
	// 	.subscribe(PrintObserver::new("error_boundary_operator (composite)"));

	let _s3 = throw("error".to_string())
		.map(|i| i)
		.into_result()
		.error_boundary()
		.subscribe(PrintObserver::new("error_boundary_operator (composite)"));
}
