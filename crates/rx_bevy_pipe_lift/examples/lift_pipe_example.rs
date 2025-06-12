use rx_bevy::prelude::*;
use rx_bevy_pipe_lift::LiftOperator;

fn main() {
	of(1)
		.pipe(LiftOperator::new(|next| of(next * 2), |_| None))
		.flat(SwitchFlattener::default())
		.subscribe(PrintObserver::new("lifted, then flattened value"));
}
