use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::MergeMapOperator;

pub trait ObservablePipeExtensionMergeMap: Observable + Sized {
	fn merge_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, MergeMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, MergeMapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionMergeMap for O where O: Observable {}
