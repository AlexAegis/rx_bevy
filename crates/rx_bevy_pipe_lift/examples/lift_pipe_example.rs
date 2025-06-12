use rx_bevy::prelude::*;
use rx_bevy_pipe_lift::{LiftOperator, prelude::ObservableExtensionLiftPipe};

fn main() {
	of(1)
		.lift(LiftOperator::new(|next| of(next * 2), |_| None))
		.flat(SwitchFlattener::new())
		.subscribe(PrintObserver::new("lifted, then flattened value"));
}
