use rx_bevy_core::{Observable, SignalBound};
use rx_bevy_ref_pipe::Pipe;

use crate::LiftResultOperator;

/// Operator creator function
pub fn lift_result<ResultIn, ResultInError, InError, InErrorToResultError>(
	in_error_to_result_error: InErrorToResultError,
) -> LiftResultOperator<ResultIn, ResultInError, InError, InErrorToResultError>
where
	ResultIn: SignalBound,
	ResultInError: SignalBound,
	InError: SignalBound,
	InErrorToResultError: Clone + Fn(InError) -> ResultInError,
{
	LiftResultOperator::new(in_error_to_result_error)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>> + Sized
where
	ResultOut: SignalBound,
	ResultOutError: SignalBound,
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
			Self::Context,
		>,
	>
	where
		InErrorToResultError: Fn(Self::OutError) -> ResultOutError + Clone + Send + Sync,
	{
		Pipe::new(self, LiftResultOperator::new(in_error_to_result_error))
	}
}

impl<Obs, ResultOut, ResultOutError> ObservableExtensionLiftResult<ResultOut, ResultOutError>
	for Obs
where
	Obs: Observable<Out = Result<ResultOut, ResultOutError>>,
	ResultOut: SignalBound,
	ResultOutError: SignalBound,
{
}
