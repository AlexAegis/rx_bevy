use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::LiftOptionOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait OperatorComposeExtensionLiftOption<T>: Operator<Out = Option<T>> + Sized
where
	T: Signal,
{
	fn lift_option(
		self,
	) -> CompositeOperator<Self, LiftOptionOperator<T, Self::OutError, Self::Context>> {
		CompositeOperator::new(self, LiftOptionOperator::default())
	}
}

impl<Op, T> OperatorComposeExtensionLiftOption<T> for Op
where
	Op: Operator<Out = Option<T>>,
	T: Signal,
{
}
