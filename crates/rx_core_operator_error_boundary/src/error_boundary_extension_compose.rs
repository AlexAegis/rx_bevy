use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Never};

use crate::operator::ErrorBoundaryOperator;

pub trait OperatorComposeExtensionErrorBoundary:
	ComposableOperator<OutError = Never> + Sized
{
	#[inline]
	fn error_boundary(self) -> CompositeOperator<Self, ErrorBoundaryOperator<Self::Out>> {
		self.compose_with(ErrorBoundaryOperator::default())
	}
}

impl<Op> OperatorComposeExtensionErrorBoundary for Op where Op: ComposableOperator<OutError = Never> {}
