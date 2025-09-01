use rx_bevy_core::Operator;

use crate::CompositeOperator;

pub trait CompositeOperatorExtension: Operator + Sized {
	fn pipe<NextOp>(self, next_operator: NextOp) -> CompositeOperator<Self, NextOp>
	where
		NextOp: Operator<In = Self::Out, InError = Self::OutError>,
	{
		CompositeOperator::new(self, next_operator)
	}
}

impl<T> CompositeOperatorExtension for T where T: Operator {}
