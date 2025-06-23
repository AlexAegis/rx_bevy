use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let _s = of("hello").subscribe(NoopObserver::default());
}
