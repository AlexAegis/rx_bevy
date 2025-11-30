use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::ScanOperator;

pub trait ObservablePipeExtensionScan: Observable + Sized {
	fn scan<
		NextOut: Signal + Clone,
		Reducer: 'static + Fn(&NextOut, Self::Out) -> NextOut + Clone + Send + Sync,
	>(
		self,
		reducer: Reducer,
		seed: NextOut,
	) -> Pipe<Self, ScanOperator<Self::Out, Self::OutError, Reducer, NextOut, Self::Context>> {
		Pipe::new(self, ScanOperator::new(reducer, seed))
	}
}

impl<O> ObservablePipeExtensionScan for O where O: Observable {}
