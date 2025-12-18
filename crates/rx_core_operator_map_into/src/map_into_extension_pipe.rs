use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapIntoOperator;

pub trait ObservablePipeExtensionMapInto: Observable + Sized {
	#[inline]
	fn map_into<NextOut: Signal, NextOutError: Signal>(
		self,
	) -> <MapIntoOperator<Self::Out, Self::OutError, NextOut, NextOutError> as Operator>::OutObservable<Self>
	where
		Self::Out: Into<NextOut>,
		Self::OutError: Into<NextOutError>,
	{
		MapIntoOperator::default().operate(self)
	}
}

impl<O> ObservablePipeExtensionMapInto for O where O: Observable {}
