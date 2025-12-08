use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::SwitchAllOperator;

pub trait ObservablePipeExtensionSwitchAll: Observable + Sized {
	fn switch_all(self) -> Pipe<Self, SwitchAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, SwitchAllOperator::default())
	}
}

impl<O> ObservablePipeExtensionSwitchAll for O where O: Observable {}
