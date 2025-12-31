use rx_core_traits::{Never, Observable, Operator, Signal};

use crate::operator::LiftResultOperator;

pub trait ObservablePipeExtensionLiftResult<'o, ResultOut, ResultOutError>:
	'o + Observable<Out = Result<ResultOut, ResultOutError>, OutError = Never> + Sized + Send + Sync
where
	ResultOut: Signal,
	ResultOutError: Signal,
{
	#[inline]
	fn lift_result(
		self,
	) -> <LiftResultOperator<ResultOut, ResultOutError, Self::OutError> as Operator<'o>>::OutObservable<
		Self,
	>{
		LiftResultOperator::default().operate(self)
	}
}

impl<'o, O, ResultOut, ResultOutError>
	ObservablePipeExtensionLiftResult<'o, ResultOut, ResultOutError> for O
where
	O: 'o + Observable<Out = Result<ResultOut, ResultOutError>, OutError = Never> + Send + Sync,
	ResultOut: Signal,
	ResultOutError: Signal,
{
}
