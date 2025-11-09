use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SignalBound};

use crate::operator::ScanOperator;

/// Operator creator function
pub fn scan<In, InError, Reducer, Out>(
	reducer: Reducer,
	seed: Out,
) -> ScanOperator<In, InError, Reducer, Out>
where
	In: SignalBound,
	InError: SignalBound,
	Reducer: Fn(&Out, In) -> Out + Send + Sync + Clone,
	Out: SignalBound + Clone,
{
	ScanOperator::new(reducer, seed)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionScan: Observable + Sized {
	fn scan<
		NextOut: SignalBound + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> Pipe<Self, ScanOperator<Self::Out, Self::OutError, Reducer, NextOut, Self::Context>> {
		Pipe::new(self, ScanOperator::new(reducer, seed))
	}
}

impl<T> ObservableExtensionScan for T where T: Observable {}
