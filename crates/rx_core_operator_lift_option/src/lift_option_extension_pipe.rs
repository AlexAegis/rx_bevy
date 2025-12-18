use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::LiftOptionOperator;

pub trait ObservablePipeExtensionLiftOption<T>: Observable<Out = Option<T>> + Sized
where
	T: Signal,
{
	#[inline]
	fn lift_option(
		self,
	) -> <LiftOptionOperator<T, Self::OutError> as Operator>::OutObservable<Self> {
		LiftOptionOperator::default().operate(self)
	}
}

impl<O, T> ObservablePipeExtensionLiftOption<T> for O
where
	O: Observable<Out = Option<T>>,
	T: Signal,
{
}
