use rx_bevy::prelude::*;

/// The map operator is used to transform incoming values into something else
fn main() {
	of(1).map(|i: i32| i + 1).subscribe(NoopObserver::new());
}
