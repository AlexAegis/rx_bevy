use rx_core_common::{Never, Observable, Operator, Signal};

use crate::operator::MapNeverErrorOperator;

pub trait ObservablePipeExtensionMapNeverError<'o>:
	'o + Observable<OutError = Never> + Sized + Send + Sync
{
	#[inline]
	fn map_never<NextOutError: Signal>(
		self,
	) -> <MapNeverErrorOperator<Self::Out, NextOutError> as Operator<'o>>::OutObservable<Self> {
		MapNeverErrorOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMapNeverError<'o> for O where
	O: 'o + Observable<OutError = Never> + Send + Sync
{
}
