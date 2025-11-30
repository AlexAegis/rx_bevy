use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::SkipOperator;

pub trait ObservablePipeExtensionSkip: Observable + Sized {
	fn skip(
		self,
		count: usize,
	) -> Pipe<Self, SkipOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, SkipOperator::new(count))
	}
}

impl<O> ObservablePipeExtensionSkip for O where O: Observable {}
