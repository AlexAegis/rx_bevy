use rx_bevy::prelude::*;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let mut iterator_observable = IteratorObservable::new(1..=3);
	iterator_observable.subscribe(PrintObserver::new("hello once"));
	iterator_observable.subscribe(PrintObserver::new("hello twice"));
}
