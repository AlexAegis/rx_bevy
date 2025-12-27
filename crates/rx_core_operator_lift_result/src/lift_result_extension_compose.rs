use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Never, Signal};

use crate::operator::LiftResultOperator;

pub trait OperatorComposeExtensionLiftResult<ResultIn, ResultInError>:
	ComposableOperator<Out = Result<ResultIn, ResultInError>, OutError = Never> + Sized
where
	ResultIn: Signal,
	ResultInError: Signal,
{
	#[inline]
	fn lift_result(
		self,
	) -> CompositeOperator<Self, LiftResultOperator<ResultIn, ResultInError, Self::OutError>> {
		self.compose_with(LiftResultOperator::default())
	}
}

impl<Op, ResultIn, ResultInError> OperatorComposeExtensionLiftResult<ResultIn, ResultInError> for Op
where
	Op: ComposableOperator<Out = Result<ResultIn, ResultInError>, OutError = Never>,
	ResultIn: Signal,
	ResultInError: Signal,
{
}
