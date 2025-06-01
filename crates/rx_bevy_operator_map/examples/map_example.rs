use rx_bevy_observable::prelude::*;
use rx_bevy_observable_of::of;
use rx_bevy_observer_fn::FnObserver;
use rx_bevy_operator_map::MapOperator;
use rx_bevy_operator_pipe::prelude::*;

/// The map operator is used to transform incoming values into something else
fn main() {
	let observable = of(12);

	let map_op = MapOperator::new(|lel: i32| lel + 1);

	let mut piped = observable.pipe(map_op);

	let print_observer = FnObserver::new(|lel: i32| println!("hello {lel}"));
	piped.subscribe(print_observer);
}
