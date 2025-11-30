use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::TakeOperator;

pub trait ObservablePipeExtensionTake: Observable + Sized {
	fn take(
		self,
		count: usize,
	) -> Pipe<Self, TakeOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, TakeOperator::new(count))
	}
}

impl<O> ObservablePipeExtensionTake for O where O: Observable {}
