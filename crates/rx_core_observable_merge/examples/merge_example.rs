use rx_core::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let observable_1 = (1..=3).into_observable().skip(2);
	let observable_2 = (4..=6).into_observable().take(1);
	let observable_3 = (95..=98).into_observable();
	let _s = merge((observable_1, observable_2, observable_3), usize::MAX)
		.subscribe(PrintObserver::<i32>::new("merge_operator"));
}
