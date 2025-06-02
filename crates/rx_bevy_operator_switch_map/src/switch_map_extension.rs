use rx_bevy_observable::Observable;
use rx_bevy_operator_pipe::Pipe;

use crate::SwitchMapOperator;

pub trait ObservableExtensionSwitchMap<Out>: Observable<Out = Out> + Sized {
	fn map<NextOut, F: Fn(Out) -> NextOut>(
		self,
		transform: F,
	) -> Pipe<
		Self,
		SwitchMapOperator<Out, NextOut, F, Self::Error>,
		Self::Error,
		Self::Error,
		Out,
		NextOut,
	> {
		Pipe::new(self, SwitchMapOperator::new(transform))
	}
}

impl<T, Out> ObservableExtensionSwitchMap<Out> for T where T: Observable<Out = Out> {}
