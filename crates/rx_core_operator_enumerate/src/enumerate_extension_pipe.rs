use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::Observable;

use crate::operator::EnumerateOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionEnumerate: Observable + Sized {
	fn enumerate(self) -> Pipe<Self, EnumerateOperator<Self::Out, Self::OutError, Self::Context>> {
		Pipe::new(self, EnumerateOperator::default())
	}
}

impl<Obs> ObservableExtensionEnumerate for Obs where Obs: Observable {}
