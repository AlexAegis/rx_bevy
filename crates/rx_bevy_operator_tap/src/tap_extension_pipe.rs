use rx_bevy_observable::Observable;
use rx_bevy_pipe::Pipe;

use crate::TapOperator;

/// Operator creator function
pub fn tap_next<In, InError, Callback>(callback: Callback) -> TapOperator<In, InError, Callback>
where
	Callback: Clone + for<'a> Fn(&'a In),
{
	TapOperator::new(callback)
}

/// Provides a convenient function to pipe the operator from an observable
pub trait ObservableExtensionTapNext: Observable + Sized {
	fn tap_next<Callback: 'static + Clone + for<'a> Fn(&'a Self::Out)>(
		self,
		callback: Callback,
	) -> Pipe<Self, TapOperator<Self::Out, Self::OutError, Callback>> {
		Pipe::new(self, TapOperator::new(callback))
	}
}

impl<T> ObservableExtensionTapNext for T where T: Observable {}
