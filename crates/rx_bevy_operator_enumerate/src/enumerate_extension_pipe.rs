use rx_bevy_core::Observable;
use rx_bevy_ref_pipe::Pipe;

use crate::EnumerateOperator;

/// Operator creator function
pub fn enumerate<In, InError>() -> EnumerateOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	EnumerateOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionEnumerate: Observable + Sized {
	fn enumerate(self) -> Pipe<Self, EnumerateOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, EnumerateOperator::default())
	}
}

impl<Obs> ObservableExtensionEnumerate for Obs where Obs: Observable {}
