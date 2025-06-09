use rx_bevy::prelude::*;

/// The FlatObserver reads the values from observer observables
fn main() {
	let shared_observer = SharedObserver::new(PrintObserver::new("asd"));

	of(of(1)).subscribe(FlatObserver::new(shared_observer));
}
