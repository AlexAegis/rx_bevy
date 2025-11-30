use rx_core::prelude::*;

/// An [IteratorObservable] turns the items from an [IntoIterator] and emits
/// them immediately upon subscription
fn main() {
	let iterator_observable = IteratorObservable::<_, ()>::new(1..=7);
	let _s = iterator_observable
		.take(4)
		.finalize(|_| println!("fin"))
		.subscribe(PrintObserver::new("iterator_observable"), &mut ());
}
