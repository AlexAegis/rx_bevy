use rx_core_traits::{Never, Observable, Operator, Signal};

use crate::operator::MapNeverNextOperator;

pub trait ObservablePipeExtensionMapNeverNext<'o>:
	'o + Observable<Out = Never> + Sized + Send + Sync
{
	#[inline]
	fn map_never<NextOut: Signal>(
		self,
	) -> <MapNeverNextOperator<NextOut, Self::OutError> as Operator<'o>>::OutObservable<Self> {
		MapNeverNextOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMapNeverNext<'o> for O where
	O: 'o + Observable<Out = Never> + Send + Sync
{
}
