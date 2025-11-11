use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::MapOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionMap: Observable + Sized {
	fn map<
		NextOut: SignalBound,
		Mapper: 'static + Fn(Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		mapper: Mapper,
	) -> Pipe<Self, MapOperator<Self::Out, Self::OutError, Mapper, NextOut, Self::Context>> {
		Pipe::new(self, MapOperator::new(mapper))
	}
}

impl<T> ObservableExtensionMap for T where T: Observable {}
