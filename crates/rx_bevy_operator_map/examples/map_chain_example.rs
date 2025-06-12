use rx_bevy_observable::prelude::*;
use rx_bevy_observable_of::of;
use rx_bevy_observer_fn::DynFnObserver;
use rx_bevy_operator_map::prelude::*;

/// The map operator is used to transform incoming values into something else
fn main() {
	let print_observer =
		DynFnObserver::default().with_next(|next: String| println!("hello {next}"));

	of(1)
		.map(|i| i + 1)
		.map(|i| i * 2)
		.map(|i| i.to_string())
		.subscribe(print_observer);
}
