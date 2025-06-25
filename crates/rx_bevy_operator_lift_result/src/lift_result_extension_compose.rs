use rx_bevy_observable::Operator;
use rx_bevy_operator_composite::CompositeOperator;

use crate::LiftResultOperator;

/// Provides a convenient function to pipe the operator from another operator  
pub trait CompositeOperatorExtensionLiftResult<ResultIn, ResultInError>:
	Operator<Out = Result<ResultIn, ResultInError>> + Sized
where
	ResultIn: 'static,
	ResultInError: 'static,
{
	fn lift_result<InErrorToResultError>(
		self,
		in_error_to_result_error: InErrorToResultError,
	) -> CompositeOperator<
		Self,
		LiftResultOperator<ResultIn, ResultInError, Self::OutError, InErrorToResultError>,
	>
	where
		InErrorToResultError: 'static + Clone + Fn(Self::OutError) -> ResultInError,
	{
		CompositeOperator::new(self, LiftResultOperator::new(in_error_to_result_error))
	}
}

impl<Op, ResultIn, ResultInError> CompositeOperatorExtensionLiftResult<ResultIn, ResultInError>
	for Op
where
	Op: Operator<Out = Result<ResultIn, ResultInError>>,
	ResultIn: 'static,
	ResultInError: 'static,
{
}
