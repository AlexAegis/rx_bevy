use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::EnumerateOperator;

/// Operator creator function
pub fn enumerate<In, InError>() -> EnumerateOperator<In, InError>
where
	In: SignalBound,
	InError: SignalBound,
{
	EnumerateOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionEnumerate: Observable + Sized {
	fn enumerate(self) -> Pipe<Self, EnumerateOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, EnumerateOperator::default())
	}
}

impl<Obs> ObservableExtensionEnumerate for Obs where Obs: Observable {}
