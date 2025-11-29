use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::ReduceOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionReduce: Observable + Sized {
	fn reduce<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> Pipe<Self, ReduceOperator<Self::Out, Self::OutError, Reducer, NextOut, Self::Context>> {
		Pipe::new(self, ReduceOperator::new(reducer, seed))
	}
}

impl<T> ObservableExtensionReduce for T where T: Observable {}
