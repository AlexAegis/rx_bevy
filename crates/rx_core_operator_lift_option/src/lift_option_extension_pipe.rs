use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::LiftOptionOperator;

pub trait ObservablePipeExtensionLiftOption<'o, T>:
	'o + Observable<Out = Option<T>> + Sized + Send + Sync
where
	T: Signal,
{
	#[inline]
	fn lift_option(
		self,
	) -> <LiftOptionOperator<T, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		LiftOptionOperator::default().operate(self)
	}
}

impl<'o, O, T> ObservablePipeExtensionLiftOption<'o, T> for O
where
	O: 'o + Observable<Out = Option<T>> + Send + Sync,
	T: Signal,
{
}
