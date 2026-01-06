use rx_core_traits::{Never, Observable, Operator, Signal};

use crate::operator::MapNeverBothOperator;

pub trait ObservablePipeExtensionMapNeverBoth<'o>:
	'o + Observable<Out = Never, OutError = Never> + Sized + Send + Sync
{
	#[inline]
	fn map_never_both<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> <MapNeverBothOperator<NextOut, NextOutError> as Operator<'o>>::OutObservable<Self> {
		MapNeverBothOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMapNeverBoth<'o> for O where
	O: 'o + Observable<Out = Never, OutError = Never> + Send + Sync
{
}
