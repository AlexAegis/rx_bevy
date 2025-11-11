use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SubscriptionContext};

use crate::operator::FinalizeOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFinalize: Observable + Sized {
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

impl<T> ObservableExtensionFinalize for T where T: Observable {}
