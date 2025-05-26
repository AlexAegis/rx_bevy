use rx_bevy::prelude::*;

/// The tap operator is used to peek inside a stream without changing its behavior
fn main() {
	let source = of(12);
	// TODO: Fix example once source is no longer part of an operator
	let operator = TapOperator::new_with_source(source, |next: &i32| {
		println!("hello {next}");
	});
	operator.subscribe(NoopObserver::new());
}
