use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ConcatMapOperator;

pub trait ObservablePipeExtensionConcatMap: Observable + Sized {
	#[inline]
	fn concat_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> <ConcatMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable> as Operator>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		ConcatMapOperator::new(mapper).operate(self)
	}
}

impl<O> ObservablePipeExtensionConcatMap for O where O: Observable {}
