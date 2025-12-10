use rx_core::prelude::*;

/// The [IntoResultOperator] is used to pack incoming values and errors into a
/// Result. When used, upstream errors are guaranteed not to reach downstream.
fn main() {
	let _s = throw("error!".to_string())
		.into_result()
		.subscribe(PrintObserver::new("into_result_operator"));
}
