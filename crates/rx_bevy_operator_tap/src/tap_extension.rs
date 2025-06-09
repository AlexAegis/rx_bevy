use rx_bevy_observable::Observable;
use rx_bevy_pipe_operator::Pipe;

use crate::TapOperator;

pub trait ObservableExtensionTap<Out>: Observable<Out = Out> + Sized {
	fn tap<Callback: Clone + for<'a> Fn(&'a Out)>(
		self,
		callback: Callback,
	) -> Pipe<Self, TapOperator<Out, Callback, Self::Error>> {
		Pipe::new(self, TapOperator::new(callback))
	}
}

impl<T, Out> ObservableExtensionTap<Out> for T where T: Observable<Out = Out> {}
