use rx_bevy_observable::Observable;
use rx_bevy_pipe_operator::Pipe;

use crate::IdentityOperator;

pub trait ObservableExtensionIdentity<Out>: Observable<Out = Out> + Sized {
	fn identity(self) -> Pipe<Self, IdentityOperator<Out, Self::Error>> {
		Pipe::new(self, IdentityOperator::new())
	}
}

impl<T, Out> ObservableExtensionIdentity<Out> for T where T: Observable<Out = Out> {}
