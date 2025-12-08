use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::Operator;

use crate::operator::SkipOperator;

pub trait OperatorComposeExtensionSkip: Operator + Sized {
	fn skip(
		self,
		count: usize,
	) -> CompositeOperator<Self, SkipOperator<Self::Out, Self::OutError>> {
		CompositeOperator::new(self, SkipOperator::new(count))
	}
}

impl<Op> OperatorComposeExtensionSkip for Op where Op: Operator {}
