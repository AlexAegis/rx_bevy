use rx_core_operator_composite::operator::CompositeOperator;
use rx_core_traits::{Operator, Signal};

use crate::operator::LiftResultOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait OperatorComposeExtensionLiftResult<ResultIn, ResultInError>:
	Operator<Out = Result<ResultIn, ResultInError>> + Sized
where
	ResultIn: Signal,
	ResultInError: Signal,
{
	fn lift_result<InErrorToResultError>(
		self,
		in_error_to_result_error: InErrorToResultError,
	) -> CompositeOperator<
		Self,
		LiftResultOperator<
			ResultIn,
			ResultInError,
			Self::OutError,
			InErrorToResultError,
			Self::Context,
		>,
	>
	where
		InErrorToResultError: 'static + Fn(Self::OutError) -> ResultInError + Clone + Send + Sync,
	{
		CompositeOperator::new(self, LiftResultOperator::new(in_error_to_result_error))
	}
}

impl<Op, ResultIn, ResultInError> OperatorComposeExtensionLiftResult<ResultIn, ResultInError> for Op
where
	Op: Operator<Out = Result<ResultIn, ResultInError>>,
	ResultIn: Signal,
	ResultInError: Signal,
{
}
