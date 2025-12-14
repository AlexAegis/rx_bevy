use rx_core::prelude::*;

use rx_core_observable_concat::observable_fn::concat;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable().skip(1);
	let observable_3 = (10..=19).into_observable().take(2);
	let _s = concat((observable_1, observable_2, observable_3))
		.subscribe(PrintObserver::<i32>::new("concat_operator"));
}
