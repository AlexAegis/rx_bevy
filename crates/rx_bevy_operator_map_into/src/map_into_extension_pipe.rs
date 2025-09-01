use rx_bevy_core::Observable;
use rx_bevy_pipe::Pipe;

use crate::MapIntoOperator;

/// Operator creator function
pub fn into<In, InError, Out, OutError>() -> MapIntoOperator<In, InError, Out, OutError> {
	MapIntoOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionInto: Observable + Sized {
	fn map_into<NextOut: 'static, NextOutError: 'static>(
		self,
	) -> Pipe<Self, MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError>>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		Pipe::new(self, MapIntoOperator::default())
	}
}

impl<T> ObservableExtensionInto for T where T: Observable {}
