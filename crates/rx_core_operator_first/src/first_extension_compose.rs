use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::FirstOperator;

pub trait OperatorComposeExtensionFirst: Operator + Sized {
	fn first(self) -> CompositeOperator<Self, FirstOperator<Self::Out, Self::OutError>>
	where
		Self::Out: Clone,
	{
		CompositeOperator::new(self, FirstOperator::default())
	}
}

impl<Op> OperatorComposeExtensionFirst for Op where Op: Operator {}
