use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::LiftResultOperator;

pub trait ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>> + Sized
where
	ResultOut: Signal,
	ResultOutError: Signal,
{
	fn lift_result<InErrorToResultError>(
		self,
		in_error_to_result_error: InErrorToResultError, // TODO: Remove this, use Into. Users should use the map_error operator when needed
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

impl<O, ResultOut, ResultOutError> ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>
	for O
where
	O: Observable<Out = Result<ResultOut, ResultOutError>>,
	ResultOut: Signal,
	ResultOutError: Signal,
{
}
