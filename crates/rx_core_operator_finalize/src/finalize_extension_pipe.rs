use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SubscriptionContext};

use crate::operator::FinalizeOperator;

pub trait ObservablePipeExtensionFinalize: Observable + Sized {
	fn finalize<
		Callback: 'static
			+ Clone
			+ FnOnce(&mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
			+ Send
			+ Sync,
	>(
		self,
		callback: Callback,
	) -> Pipe<Self, FinalizeOperator<Self::Out, Self::OutError, Callback, Self::Context>> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<O> ObservablePipeExtensionFinalize for O where O: Observable {}
