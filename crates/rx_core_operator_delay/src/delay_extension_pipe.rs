use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Signal};

use crate::operator::{DelayOperator, DelayOperatorOptions};

pub trait ObservablePipeExtensionDelay<T>: Observable<Out = T> + Sized
where
	T: Signal,
{
	fn delay(
		self,
		options: DelayOperatorOptions,
	) -> Pipe<Self, DelayOperator<T, Self::OutError, Self::Context>> {
		Pipe::new(self, DelayOperator::new(options))
	}
}

impl<O, T> ObservablePipeExtensionDelay<T> for O
where
	O: Observable<Out = T>,
	T: Signal,
{
}
