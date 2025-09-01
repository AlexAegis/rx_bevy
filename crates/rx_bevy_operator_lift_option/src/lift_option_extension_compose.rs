use rx_bevy_core::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::LiftOptionOperator;

/// Provides a convenient function to pipe the operator from another operator
pub trait CompositeOperatorExtensionLiftOption<T>: Operator<Out = Option<T>> + Sized
where
	T: 'static,
{
	fn lift_option(self) -> CompositeOperator<Self, LiftOptionOperator<T, Self::OutError>> {
		CompositeOperator::new(self, LiftOptionOperator::default())
	}
}

impl<Op, T> CompositeOperatorExtensionLiftOption<T> for Op
where
	Op: Operator<Out = Option<T>>,
	T: 'static,
{
}
