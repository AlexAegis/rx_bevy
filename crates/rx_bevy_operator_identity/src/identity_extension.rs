use rx_bevy_observable::{CompositeOperator, Observable, Operator};
use rx_bevy_pipe::Pipe;

use crate::IdentityOperator;

/// Operator creator function
pub fn identity<In, InError>() -> IdentityOperator<In, InError> {
	IdentityOperator::<In, InError>::default()
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionIdentity: Observable + Sized {
	fn identity(self) -> Pipe<Self, IdentityOperator<Self::Out, Self::OutError>> {
		Pipe::new(self, IdentityOperator::default())
	}
}

impl<T> ObservableExtensionIdentity for T where T: Observable {}

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionIdentity: Operator + Sized {
	fn identity(self) -> CompositeOperator<Self, IdentityOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, IdentityOperator::default())
	}
}

impl<T> CompositeOperatorExtensionIdentity for T where T: Operator {}
