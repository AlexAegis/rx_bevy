use rx_bevy_core::{Observable, SignalContext};
use rx_bevy_ref_pipe::Pipe;

use crate::TapNextOperator;

/// Operator creator function
pub fn tap_next<In, InError, OnNext, Context>(
	callback: OnNext,
) -> TapNextOperator<In, InError, OnNext, Context>
where
	OnNext: for<'a> Fn(&'a In, &'a mut Context) + Clone + Send + Sync,
	Context: SignalContext,
{
	TapNextOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTapNext: Observable + Sized {
	fn tap_next<
		OnNext: 'static + for<'a> Fn(&'a Self::Out, &'a mut Self::Context) + Clone + Send + Sync,
	>(
		self,
		callback: OnNext,
	) -> Pipe<Self, TapNextOperator<Self::Out, Self::OutError, OnNext, Self::Context>> {
		Pipe::new(self, TapNextOperator::new(callback))
	}
}

impl<T> ObservableExtensionTapNext for T where T: Observable {}
