use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe::Pipe;

use crate::MulticastOperator;

/// Operator creator function
pub fn multicast<In, InError>() -> MulticastOperator<In, InError>
where
	In: 'static,
	InError: 'static,
{
	MulticastOperator::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMulticast: Observable + Sized {
	fn multicast(self) -> Pipe<Self, MulticastOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, MulticastOperator::default())
	}
}

impl<T> ObservableExtensionMulticast for T where T: Observable {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionMulticast: Operator + Sized {
	fn multicast(self) -> CompositeOperator<Self, MulticastOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, MulticastOperator::default())
	}
}

impl<T> CompositeOperatorExtensionMulticast for T where T: Operator {}
