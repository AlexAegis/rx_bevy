use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::LiftResultOperator;

pub trait ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>> + Sized
where
	ResultOut: Signal,
	ResultOutError: Signal,
{
	#[inline]
	fn lift_result<InErrorToResultError>(
		self,
	in_error_to_result_error: InErrorToResultError, // TODO: Remove this, use Into. Users should use the map_error operator when needed, require upstream to have a Never error type
	) -> <
		LiftResultOperator<ResultOut, ResultOutError, Self::OutError, InErrorToResultError> as Operator
	>::OutObservable<Self>
	where
		InErrorToResultError: Fn(Self::OutError) -> ResultOutError + Clone + Send + Sync,
	{
		LiftResultOperator::new(in_error_to_result_error).operate(self)
	}
}

impl<O, ResultOut, ResultOutError> ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>
	for O
where
	O: Observable<Out = Result<ResultOut, ResultOutError>>,
	ResultOut: Signal,
	ResultOutError: Signal,
{
}
