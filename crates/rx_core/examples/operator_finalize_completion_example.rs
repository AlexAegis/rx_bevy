use rx_core::prelude::*;

/// The [FinalizeOperator]s closure will be called upon **either** when the
/// source [Observable] completes, or when the subscription gets unsubscribed.
///
/// It will only be called once per subscription and consumed!
///
/// Output:
///
/// ```sh
/// finalize_operator - next: 12
/// finalize_operator - completed
/// finally!
/// finalize_operator - unsubscribed
/// ```
fn main() {
	let _s = just(12)
		.finalize(|| println!("finally!"))
		.subscribe(PrintObserver::new("finalize_operator"));
}
