use rx_bevy_observable::Observable;
use rx_bevy_operator_pipe::Pipe;

use crate::FinalizeOperator;

pub trait ObservableExtensionFinalize<Out>: Observable<Out = Out> + Sized {
	fn finalize<Callback: FnOnce()>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Out, Callback, Self::Error>, Self::Error, Self::Error, Out, Out>
	{
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionFinalize<Out> for T where T: Observable<Out = Out> {}
