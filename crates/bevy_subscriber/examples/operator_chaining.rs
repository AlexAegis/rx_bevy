use bevy_subscriber::{
	observables::{Observable, ObservableWithOperators, of},
	observers::PrintObserver,
};

/// This is how you'd normally use operators that are extending Observable
/// to provide simple methods to create them
fn main() {
	of(12)
		.map(|n: i32| n * 2)
		.map(|n: i32| n.to_string())
		.subscribe(PrintObserver::new("hello"));
}
