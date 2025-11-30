use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::MapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservablePipeExtensionMap: Observable + Sized {
	fn map<NextOut: Signal, Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>> {
		Pipe::new(self, MapOperator::new(mapper))
	}
}

impl<O> ObservablePipeExtensionMap for O where O: Observable {}
