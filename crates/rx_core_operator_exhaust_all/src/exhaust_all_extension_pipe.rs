use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::ExhaustAllOperator;

pub trait ObservablePipeExtensionExhaustAll: Observable + Sized {
	fn exhaust_all(self) -> Pipe<Self, ExhaustAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, ExhaustAllOperator::default())
	}
}

impl<O> ObservablePipeExtensionExhaustAll for O where O: Observable {}
