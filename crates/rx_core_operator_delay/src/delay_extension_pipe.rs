use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Scheduler, Signal};

use crate::operator::{DelayOperator, DelayOperatorOptions};

pub trait ObservablePipeExtensionDelay<T, S>: Observable<Out = T> + Sized
where
	T: Signal,
	S: Scheduler,
{
	fn delay(
		self,
		options: DelayOperatorOptions<S>,
	) -> Pipe<Self, DelayOperator<T, Self::OutError, Self::Context, S>> {
		Pipe::new(self, DelayOperator::new(options))
	}
}

impl<O, T, S> ObservablePipeExtensionDelay<T, S> for O
where
	O: Observable<Out = T>,
	T: Signal,
	S: Scheduler,
{
}
