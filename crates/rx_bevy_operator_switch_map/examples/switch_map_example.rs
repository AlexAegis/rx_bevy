use rx_bevy_observable::prelude::*;
use rx_bevy_observable_of::of;
use rx_bevy_observer_fn::DynFnObserver;
use rx_bevy_operator_pipe::prelude::*;
use rx_bevy_operator_switch_map::SwitchMapOperator;

/// The map operator is used to transform incoming values into something else
fn main() {
	let observable = of(12);

	let map_op = SwitchMapOperator::new(|next: i32| next * 2);

	let piped = observable.pipe(map_op);
	let mut piped_again = piped
		.pipe(SwitchMapOperator::new(|next: i32| next.to_string()))
		.pipe(SwitchMapOperator::new(|next| {
			format!("{next} is the number")
		}));

	let print_observer = DynFnObserver::new().with_next(|next: String| println!("hello {next}"));

	piped_again.subscribe(print_observer);
}
