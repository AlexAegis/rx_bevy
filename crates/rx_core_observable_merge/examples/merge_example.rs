use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let observable_1 = (1..=3).into_observable::<()>();
	let observable_2 = (4..=6).into_observable::<()>();
	let _s = merge(observable_1, observable_2)
		.subscribe(PrintObserver::<i32, ()>::new("merge_operator"), &mut ());
}
