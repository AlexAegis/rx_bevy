use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Teardown};

use crate::operator::FinalizeOperator;

pub trait ObservablePipeExtensionFinalize: Observable + Sized {
	fn finalize<Callback: 'static + Clone + Into<Teardown> + Send + Sync>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Self::Out, Self::OutError, Callback>> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<O> ObservablePipeExtensionFinalize for O where O: Observable {}
