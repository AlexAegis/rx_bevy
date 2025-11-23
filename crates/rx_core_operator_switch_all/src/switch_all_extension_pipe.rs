use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, ObservableOutput};

use crate::operator::SwitchAllOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSwitchAll: Observable + Sized {
	fn switch_all(self) -> Pipe<Self, SwitchAllOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Observable<Context = Self::Context>,
		Self::OutError: Into<<Self::Out as ObservableOutput>::OutError>,
	{
		Pipe::new(self, SwitchAllOperator::default())
	}
}

impl<T> ObservableExtensionSwitchAll for T where T: Observable {}
