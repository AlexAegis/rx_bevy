use rx_bevy::prelude::*;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let iterator_observable = IteratorObservable::new(1..=7);
	let _s = iterator_observable
		.take(3)
		.finalize(|_| println!("fin"))
		.subscribe(PrintObserver::new("hello once"), &mut ());
}
