use rx_core_traits::{Never, Observable, Operator, Signal};

use crate::operator::LiftResultOperator;

pub trait ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>:
	Observable<Out = Result<ResultOut, ResultOutError>, OutError = Never> + Sized
where
	ResultOut: Signal,
	ResultOutError: Signal,
{
	#[inline]
	fn lift_result(
		self,
	) -> <LiftResultOperator<ResultOut, ResultOutError, Self::OutError> as Operator>::OutObservable<
		Self,
	> {
		LiftResultOperator::default().operate(self)
	}
}

impl<O, ResultOut, ResultOutError> ObservablePipeExtensionLiftResult<ResultOut, ResultOutError>
	for O
where
	O: Observable<Out = Result<ResultOut, ResultOutError>, OutError = Never>,
	ResultOut: Signal,
	ResultOutError: Signal,
{
}
