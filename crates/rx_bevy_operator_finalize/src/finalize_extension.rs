use rx_bevy_observable::Observable;
use rx_bevy_pipe_operator::Pipe;

use crate::FinalizeOperator;

pub trait ObservableExtensionFinalize<Out>: Observable<Out = Out> + Sized {
	fn finalize<Callback: Clone + FnOnce()>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Out, Callback, Self::OutError>> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionFinalize<Out> for T where T: Observable<Out = Out> {}
