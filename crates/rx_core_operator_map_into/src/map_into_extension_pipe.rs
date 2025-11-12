use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::MapIntoOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMapInto: Observable + Sized {
	fn map_into<NextOut: SignalBound, NextOutError: SignalBound>(
		self,
	) -> Pipe<Self, MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError, Self::Context>>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		Pipe::new(self, MapIntoOperator::default())
	}
}

impl<T> ObservableExtensionMapInto for T where T: Observable {}
