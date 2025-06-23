use rx_bevy_observable::Observable;
use rx_bevy_pipe::Pipe;

use crate::TakeOperator;

/// Operator creator function
pub fn take<In, InError>(count: usize) -> TakeOperator<In, InError> {
	TakeOperator::new(count)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTake: Observable + Sized {
	fn take(self, count: usize) -> Pipe<Self, TakeOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, TakeOperator::new(count))
	}
}

impl<T> ObservableExtensionTake for T where T: Observable {}
