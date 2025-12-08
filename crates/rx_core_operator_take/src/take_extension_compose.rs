use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::TakeOperator;

pub trait OperatorComposeExtensionTake: Operator + Sized {
	fn take(
		self,
		count: usize,
	) -> CompositeOperator<Self, TakeOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, TakeOperator::new(count))
	}
}

impl<Op> OperatorComposeExtensionTake for Op where Op: Operator {}
