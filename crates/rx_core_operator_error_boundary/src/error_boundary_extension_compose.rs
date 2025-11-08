use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Never, Operator};

use crate::operator::ErrorBoundaryOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionErrorBoundary: Operator<OutError = Never> + Sized {
	fn error_boundary(
		self,
	) -> CompositeOperator<Self, ErrorBoundaryOperator<Self::Out, Self::Context>> {
		CompositeOperator::new(self, ErrorBoundaryOperator::default())
	}
}

impl<T> CompositeOperatorExtensionErrorBoundary for T where T: Operator<OutError = Never> {}
