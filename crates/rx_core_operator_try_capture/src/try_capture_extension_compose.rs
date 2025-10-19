use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::TryCaptureOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionTryCapture: Operator + Sized {
	fn lift_result(
		self,
	) -> CompositeOperator<
		Self,
		TryCaptureOperator<Self::Out, Self::OutError, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, TryCaptureOperator::default())
	}
}

impl<Op> CompositeOperatorExtensionTryCapture for Op where Op: Operator {}
