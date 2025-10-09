use rx_bevy_core::{Observable, WithContext};
use rx_bevy_ref_pipe::Pipe;

use crate::FinalizeOperator;

/// Operator creator function
pub fn finalize<Out, OutError, Callback, Context>(
	callback: Callback,
) -> FinalizeOperator<Out, OutError, Callback, Context>
where
	Callback: 'static + Clone + FnOnce(&mut Context),
{
	FinalizeOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionFinalize: Observable + Sized {
	fn finalize<
		Callback: 'static + Clone + FnOnce(&mut <Self::Subscription as WithContext>::Context),
	>(
		self,
		callback: Callback,
	) -> Pipe<
		Self,
		FinalizeOperator<
			Self::Out,
			Self::OutError,
			Callback,
			<Self::Subscription as WithContext>::Context,
		>,
	> {
		Pipe::new(self, FinalizeOperator::new(callback))
	}
}

impl<T> ObservableExtensionFinalize for T where T: Observable {}
