use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, SignalBound};

use crate::operator::LiftOptionOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionLiftOption<T>: Operator<Out = Option<T>> + Sized
where
	T: SignalBound,
{
	fn lift_option(
		self,
	) -> CompositeOperator<Self, LiftOptionOperator<T, Self::OutError, Self::Context>> {
		CompositeOperator::new(self, LiftOptionOperator::default())
	}
}

impl<Op, T> CompositeOperatorExtensionLiftOption<T> for Op
where
	Op: Operator<Out = Option<T>>,
	T: SignalBound,
{
}
