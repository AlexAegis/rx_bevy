use rx_bevy_observable::Observable;
use rx_bevy_operator::Operator;

use crate::Pipe;

pub trait ObservableExtensionPipe: Observable + Sized {
	fn pipe<Op>(self, operator: Op) -> Pipe<Self, Op>
	where
		Self: Sized,
		Op: Operator,
	{
		Pipe::new(self, operator)
	}
}

impl<T> ObservableExtensionPipe for T where T: Observable {}
