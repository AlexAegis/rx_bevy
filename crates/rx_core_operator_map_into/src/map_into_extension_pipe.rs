use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapIntoOperator;

pub trait ObservablePipeExtensionMapInto<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn map_into<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> <MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError> as Operator<'o>>::OutObservable<Self>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		MapIntoOperator::default().operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMapInto<'o> for O where O: 'o + Observable + Send + Sync {}
