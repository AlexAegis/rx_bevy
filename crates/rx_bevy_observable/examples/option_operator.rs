use rx_bevy::prelude::*;

fn main() {
	let mut optional_map_operator = Some(MapOperator::new(|i: i32| i * 2));
	optional_map_operator.take();

	(1..10)
		.into_observable()
		.pipe(optional_map_operator)
		.subscribe(PrintObserver::new("hello"));
}
