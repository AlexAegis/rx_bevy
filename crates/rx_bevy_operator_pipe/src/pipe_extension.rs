use rx_bevy_observable::Observable;
use rx_bevy_operator::Operator;

use crate::Pipe;

pub trait ObservableExtensionPipe<Out>: Observable<Out = Out> + Sized {
	fn pipe<Op>(self, operator: Op) -> Pipe<Self, Op, Out, Op::Out>
	where
		Self: Sized,
		Op: Operator,
	{
		Pipe::new(self, operator)
	}
}

impl<T, Out> ObservableExtensionPipe<Out> for T where T: Observable<Out = Out> {}
