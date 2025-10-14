use rx_bevy_core::{Observable, SignalBound, context::SubscriptionContext};
use rx_bevy_observable_pipe::Pipe;

use crate::TapNextOperator;

/// Operator creator function
pub fn tap_next<In, InError, OnNext, Context>(
	callback: OnNext,
) -> TapNextOperator<In, InError, OnNext, Context>
where
	In: SignalBound,
	InError: SignalBound,
	OnNext: 'static + Fn(&In, &mut Context::Item<'_>) + Clone + Send + Sync,
	Context: SubscriptionContext,
{
	TapNextOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTapNext: Observable + Sized {
	fn tap_next<
		OnNext: 'static
			+ Fn(&Self::Out, &mut <Self::Context as SubscriptionContext>::Item<'_>)
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
