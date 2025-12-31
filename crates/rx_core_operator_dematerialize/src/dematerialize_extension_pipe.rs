use rx_core_traits::{Never, Observable, ObserverNotification, Operator, Signal};

use crate::operator::DematerializeOperator;

pub trait ObservablePipeExtensionDematerialize<'o, In, InError>:
	'o + Observable<Out = ObserverNotification<In, InError>, OutError = Never> + Sized + Send + Sync
where
	In: Signal,
	InError: Signal,
{
	#[inline]
	fn dematerialize(
		self,
	) -> <DematerializeOperator<In, InError> as Operator<'o>>::OutObservable<Self> {
		DematerializeOperator::<In, InError>::default().operate(self)
	}
}

impl<'o, O, In, InError> ObservablePipeExtensionDematerialize<'o, In, InError> for O
where
	O: 'o + Observable<Out = ObserverNotification<In, InError>, OutError = Never> + Send + Sync,
	In: Signal,
	InError: Signal,
{
}
