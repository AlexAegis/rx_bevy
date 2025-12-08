use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Never, Operator};

use crate::operator::ErrorBoundaryOperator;

pub trait OperatorComposeExtensionErrorBoundary: Operator<OutError = Never> + Sized {
	fn error_boundary(self) -> CompositeOperator<Self, ErrorBoundaryOperator<Self::Out>> {
		CompositeOperator::new(self, ErrorBoundaryOperator::default())
	}
}

impl<Op> OperatorComposeExtensionErrorBoundary for Op where Op: Operator<OutError = Never> {}
