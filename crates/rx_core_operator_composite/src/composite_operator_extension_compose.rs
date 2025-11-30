use rx_core_traits::Operator;

use crate::operator::CompositeOperator;

pub trait OperatorComposeExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		CompositeOperator::new(self, next_operator)
	}
}

impl<Op> OperatorComposeExtension for Op where Op: Operator {}
