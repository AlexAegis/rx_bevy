use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Signal};

use crate::operator::LiftOptionOperator;

pub trait OperatorComposeExtensionLiftOption<T>:
	ComposableOperator<Out = Option<T>> + Sized
where
	T: Signal,
{
	#[inline]
	fn lift_option(self) -> CompositeOperator<Self, LiftOptionOperator<T, Self::OutError>> {
		self.compose_with(LiftOptionOperator::default())
	}
}

impl<Op, T> OperatorComposeExtensionLiftOption<T> for Op
where
	Op: ComposableOperator<Out = Option<T>>,
	T: Signal,
{
}
