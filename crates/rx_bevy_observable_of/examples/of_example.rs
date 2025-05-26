use rx_bevy::prelude::*;
use rx_bevy_observable_of::of;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	of("hello").subscribe(NoopObserver::new());
}
