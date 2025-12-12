use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::FirstOperator;

pub trait ObservablePipeExtensionFirst: Observable + Sized {
	fn first(self) -> Pipe<Self, FirstOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		Pipe::new(self, FirstOperator::default())
	}
}

impl<O> ObservablePipeExtensionFirst for O where O: Observable {}
