use rx_core_common::ComposableOperator;

use crate::operator::CompositeOperator;

pub trait OperatorComposeExtension: ComposableOperator + Sized {
	#[inline]
	fn compose_with<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: ComposableOperator<In = Self::Out, InError = Self::OutError>,
	{
		CompositeOperator::new(self, next_operator)
	}
}

impl<Op> OperatorComposeExtension for Op where Op: ComposableOperator {}
