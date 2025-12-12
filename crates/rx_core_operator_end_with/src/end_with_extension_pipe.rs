use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::EndWithOperator;

pub trait ObservablePipeExtensionEndWith: Observable + Sized {
	fn end_with(self, end_with: Self::Out) -> Pipe<Self, EndWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		Pipe::new(self, EndWithOperator::new(end_with))
	}
}

impl<O> ObservablePipeExtensionEndWith for O where O: Observable {}
