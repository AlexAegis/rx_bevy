use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::EnumerateOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionEnumerate: Operator + Sized {
	fn enumerate(
		self,
	) -> CompositeOperator<
		Self,
		EnumerateOperator<Self::Out, Self::OutError, <Self as Operator>::Context>,
	> {
		CompositeOperator::new(self, EnumerateOperator::default())
	}
}

impl<Op> CompositeOperatorExtensionEnumerate for Op where Op: Operator {}
