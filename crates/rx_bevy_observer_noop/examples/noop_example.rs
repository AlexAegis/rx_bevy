use rx_bevy::prelude::*;

/// The NoopObserver does nothing with the received values
fn main() {
	of(1).subscribe(NoopObserver::new());
}
