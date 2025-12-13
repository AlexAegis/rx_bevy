use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::ConcatAllOperator;

pub trait ObservablePipeExtensionConcatAll: Observable + Sized {
	fn concat_all(self) -> Pipe<Self, ConcatAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, ConcatAllOperator::default())
	}
}

impl<O> ObservablePipeExtensionConcatAll for O where O: Observable {}
