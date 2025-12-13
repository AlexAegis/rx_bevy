use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::ConcatMapOperator;

pub trait ObservablePipeExtensionConcatMap: Observable + Sized {
	fn concat_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + Fn(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, ConcatMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, ConcatMapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionConcatMap for O where O: Observable {}
