use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::EnumerateOperator;

pub trait ObservablePipeExtensionEnumerate: Observable + Sized {
	fn enumerate(self) -> Pipe<Self, EnumerateOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, EnumerateOperator::default())
	}
}

impl<O> ObservablePipeExtensionEnumerate for O where O: Observable {}
