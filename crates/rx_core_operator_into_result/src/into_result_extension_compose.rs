use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::IntoResultOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionIntoResult: Operator + Sized {
	fn lift_result(
		self,
	) -> CompositeOperator<
		Self,
		IntoResultOperator<Self::Out, Self::OutError, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, IntoResultOperator::default())
	}
}

impl<Op> CompositeOperatorExtensionIntoResult for Op where Op: Operator {}
