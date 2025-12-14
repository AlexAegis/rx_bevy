use rx_core::prelude::*;

/// The [ZipObservable] combines values from multiple observables, grouping
/// their emissions in the order they were emitted. That is, the first emission
/// of the first observable will only ever be seen together with the first
/// emission of the second observable. And their second emissions will too appear
/// together and so on.
fn main() {
	let observable_1 = (1..=3).into_observable();
	let observable_2 = (4..=6).into_observable();
	// TODO: Refactor to use variadic generics/or fix the flag drop panic
	let _s = zip(observable_1, observable_2).subscribe(PrintObserver::new("zip_observable"));
}
