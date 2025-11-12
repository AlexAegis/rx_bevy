use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::SkipOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSkip: Observable + Sized {
	fn skip(
		self,
		count: usize,
	) -> Pipe<Self, SkipOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, SkipOperator::new(count))
	}
}

impl<T> ObservableExtensionSkip for T where T: Observable {}
