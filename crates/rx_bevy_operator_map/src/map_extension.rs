use rx_bevy_observable::Observable;
use rx_bevy_operator_pipe::Pipe;

use crate::MapOperator;

pub trait ObservableExtensionMap<Out>: Observable<Out = Out> + Sized {
	fn map<NextOut, F: Fn(Out) -> NextOut>(
		self,
		transform: F,
	) -> Pipe<Self, MapOperator<Out, NextOut, F, Self::Error>, Self::Error, Self::Error, Out, NextOut>
	{
		Pipe::new(self, MapOperator::new(transform))
	}
}

impl<T, Out> ObservableExtensionMap<Out> for T where T: Observable<Out = Out> {}
