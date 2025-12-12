use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::StartWithOperator;

pub trait ObservablePipeExtensionStartWith: Observable + Sized {
	fn start_with(
		self,
		start_with: Self::Out,
	) -> Pipe<Self, StartWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		Pipe::new(self, StartWithOperator::new(start_with))
	}
}

impl<O> ObservablePipeExtensionStartWith for O where O: Observable {}
