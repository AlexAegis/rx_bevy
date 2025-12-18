use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ExhaustMapOperator;

pub trait ObservablePipeExtensionExhaustMap: Observable + Sized {
	#[inline]
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> <ExhaustMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable> as Operator>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		ExhaustMapOperator::new(mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionExhaustMap for O where O: Observable {}
