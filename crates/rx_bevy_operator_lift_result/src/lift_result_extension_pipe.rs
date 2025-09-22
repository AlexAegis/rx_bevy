use rx_bevy_core::{Observable, SignalContext};
use rx_bevy_ref_pipe::Pipe;

use crate::LiftResultOperator;

/// Operator creator function
pub fn lift_result<ResultIn, ResultInError, InError, InErrorToResultError>(
	in_error_to_result_error: InErrorToResultError,
) -> LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: 'static,
	ResultInError: 'static,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	LiftResultOperator::new(in_error_to_result_error)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>> + Sized
where
	ResultOut: 'static,
	ResultOutError: 'static,
{
	fn lift_result<InErrorToResultError>(
		self,
		in_error_to_result_error: InErrorToResultError,
	) -> Pipe<
		Self,
		LiftResultOperator<
			ResultOut,
			ResultOutError,
			Self::OutError,
			InErrorToResultError,
			<Self::Subscription as SignalContext>::Context,
		>,
	>
	where
		InErrorToResultError: Clone + Fn(Self::OutError) -> ResultOutError,
	{
		Pipe::new(self, LiftResultOperator::new(in_error_to_result_error))
	}
}

impl<Obs, ResultOut, ResultOutError> ObservableExtensionLiftResult<ResultOut, ResultOutError>
	for Obs
where
	Obs: Observable<Out = Result<ResultOut, ResultOutError>>,
	ResultOut: 'static,
	ResultOutError: 'static,
{
}
