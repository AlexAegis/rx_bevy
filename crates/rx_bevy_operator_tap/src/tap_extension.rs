use rx_bevy_observable::Observable;
use rx_bevy_operator::Pipe;

use crate::TapOperator;

pub trait ObservableExtensionTap<Out>: Observable<Out = Out> + Sized {
	fn tap<Callback: for<'a> Fn(&'a Out)>(
		self,
		callback: Callback,
	) -> Pipe<Self, TapOperator<Out, Callback>, Out, Out> {
		Pipe::new(self, TapOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionTap<Out> for T where T: Observable<Out = Out> {}
