use rx_bevy_core::Observable;
use rx_bevy_pipe::Pipe;

use crate::FinalizeOperator;

/// Operator creator function
pub fn finalize<Out, OutError, Callback>(
	callback: Callback,
) -> FinalizeOperator<Out, OutError, Callback>
where
	Callback: Clone + FnOnce(),
{
	FinalizeOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFinalize: Observable + Sized {
	fn finalize<Callback: 'static + Clone + FnOnce()>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Self::Out, Self::OutError, Callback>> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<T> ObservableExtensionFinalize for T where T: Observable {}
