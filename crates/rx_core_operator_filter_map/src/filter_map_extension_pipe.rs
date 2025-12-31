use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::FilterMapOperator;

pub trait ObservablePipeExtensionFilterMap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn filter_map<
		NextOut: Signal,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> <FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut> as Operator<'o>>::OutObservable<
		Self,
	>{
		FilterMapOperator::new(mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionFilterMap<'o> for O where O: 'o + Observable + Send + Sync {}
