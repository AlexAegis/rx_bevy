use std::time::Duration;

use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::DelayOperator;

pub trait ObservablePipeExtensionDelay<'o, T, S>:
	'o + Observable<Out = T> + Sized + Send + Sync
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [DelayOperator]
	///
	/// The `delay` operator shifts upstream values forward in time by a specified
	/// duration.
	///
	/// Upstream completion and cancellation can happen instantly if there are no
	/// pending delayed values, otherwise it will complete or cancel once all
	/// delayed values have been emitted.
	///
	/// Upstream errors are immediately propagated downstream, cancelling any
	/// pending delayed values.
	#[inline]
	fn delay(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> <DelayOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		DelayOperator::new(duration, scheduler).operate(self)
	}
}

impl<'o, O, T, S> ObservablePipeExtensionDelay<'o, T, S> for O
where
	O: 'o + Observable<Out = T> + Send + Sync,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
