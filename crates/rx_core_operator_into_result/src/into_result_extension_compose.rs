use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::IntoResultOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait OperatorComposeExtensionIntoResult: Operator + Sized {
	fn lift_result(
		self,
	) -> CompositeOperator<Self, IntoResultOperator<Self::Out, Self::OutError, Self::Context>> {
		CompositeOperator::new(self, IntoResultOperator::default())
	}
}

impl<Op> OperatorComposeExtensionIntoResult for Op where Op: Operator {}
