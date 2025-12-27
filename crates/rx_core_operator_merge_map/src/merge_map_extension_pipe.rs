use rx_core_traits::{Observable, Operator, Signal};

use crate::operator::MergeMapOperator;

pub trait ObservablePipeExtensionMergeMap: Observable + Sized {
	#[inline]
	fn merge_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
				ErrorMapper: 'static + Fn(Self::OutError) -> NextInnerObservable::OutError + Clone + Send + Sync,

	>(
		self,
		mapper: Mapper,
		concurrency_limit: usize,
		error_mapper: ErrorMapper,
	) -> <MergeMapOperator<Self::Out, Self::OutError, Mapper, ErrorMapper, NextInnerObservable> as Operator>::OutObservable<Self>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		MergeMapOperator::new(mapper, error_mapper, concurrency_limit).operate(self)
	}
}

impl<O> ObservablePipeExtensionMergeMap for O where O: Observable {}
