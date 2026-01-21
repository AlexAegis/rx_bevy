use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::ObserveOnOperator;

pub trait ObservablePipeExtensionObserveOn<'o, T, S>:
	'o + Observable<Out = T> + Sized + Send + Sync
where
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [ObserveOnOperator]
	///
	/// The `observe_on` operator re-emits upstream `next` signals on the provided
	/// scheduler.
	///
	/// Upstream completion and cancellation happen immediately when there are no
	/// pending scheduled values, otherwise they are deferred until scheduled work
	/// drains.
	///
	/// Upstream errors are forwarded immediately; any pending scheduled values are
	/// skipped because downstream closes.
	#[inline]
	fn observe_on(
		self,
		scheduler: SchedulerHandle<S>,
	) -> <ObserveOnOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		ObserveOnOperator::new(scheduler).operate(self)
	}
}

impl<'o, O, T, S> ObservablePipeExtensionObserveOn<'o, T, S> for O
where
	O: 'o + Observable<Out = T> + Send + Sync,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
}
