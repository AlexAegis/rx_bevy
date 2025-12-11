use std::time::Duration;

use rx_core_observable_pipe::observable::Pipe;
use rx_core_traits::{Observable, Scheduler, SchedulerHandle, Signal};

use crate::operator::DelayOperator;

pub trait ObservablePipeExtensionDelay<T, S>: Observable<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	fn delay(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> Pipe<Self, DelayOperator<T, Self::OutError, S>> {
		Pipe::new(self, DelayOperator::new(duration, scheduler))
	}
}

impl<O, T, S> ObservablePipeExtensionDelay<T, S> for O
where
	O: Observable<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
