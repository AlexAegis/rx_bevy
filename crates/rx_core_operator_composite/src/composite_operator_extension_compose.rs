use rx_core_traits::Operator;

use crate::operator::CompositeOperator;

pub trait CompositeOperatorExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		CompositeOperator::new(self, next_operator)
	}
}

impl<T> CompositeOperatorExtension for T where T: Operator {}
