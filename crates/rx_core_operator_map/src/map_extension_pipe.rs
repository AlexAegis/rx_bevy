use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapOperator;

pub trait ObservablePipeExtensionMap: Observable + Sized {
	#[inline]
	fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
		self,
		mapper: Mapper,
	) -> <MapOperator<Self::Out, Self::OutError, Mapper, NextOut> as Operator>::OutObservable<Self>
	{
		MapOperator::new(mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionMap for O where O: Observable {}
