use rx_core_traits::{Observable, SignalBound, prelude::SubscriptionContext};
use rx_core_observable_pipe::Pipe;

use crate::FinalizeOperator;

/// Operator creator function
pub fn finalize<Out, OutError, Callback, Context>(
	callback: Callback,
) -> FinalizeOperator<Out, OutError, Callback, Context>
where
	Out: SignalBound,
	OutError: SignalBound,
	Callback: 'static + Clone + FnOnce(&mut Context::Item<'_, '_>) + Send + Sync,
	Context: SubscriptionContext,
{
	FinalizeOperator::new(callback)
}

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
