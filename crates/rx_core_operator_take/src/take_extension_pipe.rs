use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::TakeOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTake: Observable + Sized {
	fn take(
		self,
		count: usize,
	) -> Pipe<Self, TakeOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, TakeOperator::new(count))
	}
}

impl<T> ObservableExtensionTake for T where T: Observable {}
