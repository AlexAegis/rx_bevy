use rx_core_traits::{Never, Observable, ObserverNotification, Operator, Signal};

use crate::operator::DematerializeOperator;

pub trait ObservablePipeExtensionDematerialize<In, InError>:
	Observable<Out = ObserverNotification<In, InError>, OutError = Never> + Sized
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn dematerialize(
		self,
	) -> <DematerializeOperator<In, InError> as Operator>::OutObservable<Self> {
		DematerializeOperator::<In, InError>::default().operate(self)
	}
}

impl<O, In, InError> ObservablePipeExtensionDematerialize<In, InError> for O
where
	O: Observable<Out = ObserverNotification<In, InError>, OutError = Never>,
	In: Signal,
	InError: Signal,
{
}
