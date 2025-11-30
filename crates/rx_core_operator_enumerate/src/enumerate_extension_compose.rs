use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::EnumerateOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait OperatorComposeExtensionEnumerate: Operator + Sized {
	fn enumerate(
		self,
	) -> CompositeOperator<Self, EnumerateOperator<Self::Out, Self::OutError, Self::Context>> {
		CompositeOperator::new(self, EnumerateOperator::default())
	}
}

impl<Op> OperatorComposeExtensionEnumerate for Op where Op: Operator {}
