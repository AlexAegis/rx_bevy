use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::EndWithOperator;

pub trait OperatorComposeExtensionEndWith: Operator + Sized {
	fn end_with(
		self,
		end_with: Self::Out,
	) -> CompositeOperator<Self, EndWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		CompositeOperator::new(self, EndWithOperator::new(end_with))
	}
}

impl<Op> OperatorComposeExtensionEndWith for Op where Op: Operator {}
