use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::FilterMapOperator;

pub trait ObservablePipeExtensionFilterMap: Observable + Sized {
	#[inline]
	fn filter_map<
		NextOut: Signal,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> <FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut> as Operator>::OutObservable<
		Self,
	> {
		FilterMapOperator::new(mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionFilterMap for O where O: Observable {}
