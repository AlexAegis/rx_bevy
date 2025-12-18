use std::time::Duration;

use rx_core_traits::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::DelayOperator;

pub trait ObservablePipeExtensionDelay<T, S>: Observable<Out = T> + Sized
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	#[inline]
	fn delay(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> <DelayOperator<T, Self::OutError, S> as Operator>::OutObservable<Self> {
		DelayOperator::new(duration, scheduler).operate(self)
	}
}

impl<O, T, S> ObservablePipeExtensionDelay<T, S> for O
where
	O: Observable<Out = T>,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
