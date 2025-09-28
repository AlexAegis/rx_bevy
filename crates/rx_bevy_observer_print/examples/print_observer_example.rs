use rx_bevy::prelude::*;

/// The [PrintObserver] prints incoming next, error and complete notifications
/// with the supplied message prepended to it
fn main() {
	let _s = of(1).subscribe(PrintObserver::new("hello"), &mut ());
}
