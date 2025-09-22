use rx_bevy::prelude::*;

/// The [FinalizeOperator]s closure will be called upon **either** when the
/// source [Observable] completes, or when the subscription gets unsubscribed.
///
/// It will only be called once per subscription and consumed!
///
/// Output:
///
/// ```sh
/// finalize_example - next: 12
/// finalize_example - completed
/// finally!
/// ```
fn main() {
	let _s = of(12)
		.finalize(|| println!("finally!"))
		.subscribe(PrintObserver::new("finalize_operator"), &mut ());
}
