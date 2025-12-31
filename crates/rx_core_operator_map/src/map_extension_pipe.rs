use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MapOperator;

pub trait ObservablePipeExtensionMap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
		self,
		mapper: Mapper,
	) -> <MapOperator<Self::Out, Self::OutError, Mapper, NextOut> as Operator<'o>>::OutObservable<
		Self,
	> {
		MapOperator::new(mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMap<'o> for O where O: 'o + Observable + Send + Sync {}
