use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::IntoResultOperator;

pub trait ObservablePipeExtensionTryCapture: Observable + Sized {
	fn into_result(self) -> Pipe<Self, IntoResultOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, IntoResultOperator::default())
	}
}

impl<O> ObservablePipeExtensionTryCapture for O where O: Observable {}
