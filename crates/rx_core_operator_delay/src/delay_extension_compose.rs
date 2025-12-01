use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::{DelayOperator, DelayOperatorOptions};

pub trait OperatorComposeExtensionDelay<T>: Operator<Out = T> + Sized
where
	T: Signal,
{
	fn delay(
		self,
		options: DelayOperatorOptions,
	) -> CompositeOperator<Self, DelayOperator<T, Self::OutError, Self::Context>> {
		CompositeOperator::new(self, DelayOperator::new(options))
	}
}

impl<Op, T> OperatorComposeExtensionDelay<T> for Op
where
	Op: Operator<Out = T>,
	T: Signal,
{
}
