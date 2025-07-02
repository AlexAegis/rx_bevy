use rx_bevy::prelude::*;

/// The [ZipObservable] combines values from multiple observables, grouping
/// their emissions in the order they were emitted. That is, the first emission
/// of the first observable will only ever be seen together with the first
/// emission of the second observable. And their second emissions will too appear
/// together and so on.
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	let _s = zip(observable_1, observable_2).subscribe(
		DynFnObserver::default()
			.with_next(|next: (i32, i32)| println!("zip_next {}, {}", next.0, next.1))
			.with_complete(|| println!("zip_complete"))
			.with_unsubscribe(|| println!("zip_unsubscribe")),
	);
}
