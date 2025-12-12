use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::StartWithOperator;

pub trait OperatorComposeExtensionStartWith: Operator + Sized {
	fn start_with(
		self,
		start_with: Self::Out,
	) -> CompositeOperator<Self, StartWithOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		CompositeOperator::new(self, StartWithOperator::new(start_with))
	}
}

impl<Op> OperatorComposeExtensionStartWith for Op where Op: Operator {}
