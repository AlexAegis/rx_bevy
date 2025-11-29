use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::LiftResultOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>> + Sized
where
	ResultOut: Signal,
	ResultOutError: Signal,
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
	ResultOut: Signal,
	ResultOutError: Signal,
{
}
