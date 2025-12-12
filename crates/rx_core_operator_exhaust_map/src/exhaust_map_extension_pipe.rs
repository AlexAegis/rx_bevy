use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::ExhaustMapOperator;

pub trait ObservablePipeExtensionExhaustMap: Observable + Sized {
	fn exhaust_map<
		NextInnerObservable: Observable + Signal,
		Mapper: 'static + FnMut(Self::Out) -> NextInnerObservable + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, ExhaustMapOperator<Self::Out, Self::OutError, Mapper, NextInnerObservable>>
	where
		Self::OutError: Into<NextInnerObservable::OutError>,
	{
		Pipe::new(self, ExhaustMapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionExhaustMap for O where O: Observable {}
