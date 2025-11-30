use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::FilterMapOperator;

pub trait ObservablePipeExtensionFilterMap: Observable + Sized {
	fn filter_map<
		NextOut: Signal,
		Mapper: 'static + Fn(Self::Out) -> Option<NextOut> + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, FilterMapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>> {
		Pipe::new(self, FilterMapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionFilterMap for O where O: Observable {}
