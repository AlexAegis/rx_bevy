use rx_core_common::{Observable, Operator, Signal};

use crate::operator::MergeMapOperator;

pub trait ObservablePipeExtensionMergeMap<'o>: 'o + Observable + Sized + Send + Sync {
	#[inline]
	fn merge_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
				ErrorMapper: 'static + FnOnce(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,

	>(
		self,
		mapper: Mapper,
		concurrency_limit: usize,
		error_mapper: ErrorMapper,
	) -> <MergeMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable> as Operator<'o>>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		MergeMapOperator::new(mapper, error_mapper, concurrency_limit).operate(self)
	}
}

impl<'o, O> ObservablePipeExtensionMergeMap<'o> for O where O: 'o + Observable + Send + Sync {}
