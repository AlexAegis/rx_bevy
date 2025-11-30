use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::LiftOptionOperator;

pub trait ObservablePipeExtensionLiftOption<T>: Observable<Out = Option<T>> + Sized
where
	T: Signal,
{
	fn lift_option(self) -> Pipe<Self, LiftOptionOperator<T, Self::OutError, Self::Context>> {
		Pipe::new(self, LiftOptionOperator::default())
	}
}

impl<O, T> ObservablePipeExtensionLiftOption<T> for O
where
	O: Observable<Out = Option<T>>,
	T: Signal,
{
}
