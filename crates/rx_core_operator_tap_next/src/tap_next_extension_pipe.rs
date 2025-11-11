use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, SubscriptionContext};

use crate::operator::TapNextOperator;

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTapNext: Observable + Sized {
	fn tap_next<
		OnNext: 'static
			+ Fn(&Self::Out, &mut <Self::Context as SubscriptionContext>::Item<'_, '_>)
			+ Clone
			+ Send
			+ Sync,
	>(
		self,
		callback: OnNext,
	) -> Pipe<Self, TapNextOperator<Self::Out, Self::OutError, OnNext, Self::Context>> {
		Pipe::new(self, TapNextOperator::new(callback))
	}
}

impl<T> ObservableExtensionTapNext for T where T: Observable {}
