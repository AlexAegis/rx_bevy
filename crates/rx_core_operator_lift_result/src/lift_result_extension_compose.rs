use rx_core_operator_composite::{OperatorComposeExtension, operator::CompositeOperator};
use rx_core_traits::{ComposableOperator, Signal};

use crate::operator::LiftResultOperator;

pub trait OperatorComposeExtensionLiftResult<ResultIn, ResultInError>:
	ComposableOperator<Out = Result<ResultIn, ResultInError>> + Sized
where
	ResultIn: Signal,
	ResultInError: Signal,
{
	#[inline]
	fn lift_result<InErrorToResultError>(
		self,
		in_error_to_result_error: InErrorToResultError,
	) -> CompositeOperator<
		Self,
		LiftResultOperator<ResultIn, ResultInError, Self::OutError, InErrorToResultError>,
	>
	where
		InErrorToResultError: 'static + Fn(Self::OutError) -> ResultInError + Clone + Send + Sync,
	{
		self.compose_with(LiftResultOperator::new(in_error_to_result_error))
	}
}

impl<Op, ResultIn, ResultInError> OperatorComposeExtensionLiftResult<ResultIn, ResultInError> for Op
where
	Op: ComposableOperator<Out = Result<ResultIn, ResultInError>>,
	ResultIn: Signal,
	ResultInError: Signal,
{
}
