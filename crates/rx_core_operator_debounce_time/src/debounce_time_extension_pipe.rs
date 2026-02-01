use std::time::Duration;

use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::DebounceTimeOperator;

pub trait ObservablePipeExtensionDebounceTime<'o, T, S>:
	'o + Observable<Out = T> + Sized + Send + Sync
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [DebounceTimeOperator]
	///
	/// The `debounce_time` operator emits the most recent upstream value only
	/// after the specified duration passes without another emission.
	///
	/// Upstream completion and cancellation can happen instantly if there are
	/// no pending debounced values, otherwise it will complete or cancel once
	/// the pending debounced value has been emitted.
	///
	/// Upstream errors are immediately propagated downstream, cancelling any
	/// pending debounced value.
	#[inline]
	fn debounce_time(
		self,
		duration: Duration,
		scheduler: SchedulerHandle<S>,
	) -> <DebounceTimeOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		DebounceTimeOperator::new(duration, scheduler).operate(self)
	}
}

impl<'o, O, T, S> ObservablePipeExtensionDebounceTime<'o, T, S> for O
where
	O: 'o + Observable<Out = T> + Send + Sync,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
