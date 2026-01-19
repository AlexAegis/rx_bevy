use rx_core::prelude::*;

fn main() {
	let _s = throw("error!".to_string())
		.into_result()
		.subscribe(PrintObserver::new("into_result_operator - throw"));

	let _s = just(1)
		.into_result()
		.subscribe(PrintObserver::new("into_result_operator - just"));
}
