use rx_bevy::prelude::*;

fn main() {
	IteratorObservable::new(1..=10).subscribe(PrintObserver::new("hello"));
}
