use rx_bevy_core::Observable;
use rx_bevy_ref_pipe::Pipe;

use crate::SkipOperator;

/// Operator creator function
pub fn skip<In, InError>(count: usize) -> SkipOperator<In, InError> {
	SkipOperator::new(count)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionSkip: Observable + Sized {
	fn skip(
		self,
		count: usize,
	) -> Pipe<
		Self,
		SkipOperator<
			Self::Out,
			Self::OutError,
			<Self::Subscription as rx_bevy_core::WithContext>::Context,
		>,
	> {
		Pipe::new(self, SkipOperator::new(count))
	}
}

impl<T> ObservableExtensionSkip for T where T: Observable {}
