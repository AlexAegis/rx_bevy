use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::ConcatMapOperator;

pub trait ObservablePipeExtensionConcatMap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn concat_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
		ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
		error_mapper: ErrorMapper,
	) -> <ConcatMapOperator<Self::Out, Self::OutError, Mapper,ErrorMapper, NextInnerObservable> as Operator<'o>>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		ConcatMapOperator::new(mapper, error_mapper).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionConcatMap<'o> for O where O: 'o + Observable + Send + Sync {}
