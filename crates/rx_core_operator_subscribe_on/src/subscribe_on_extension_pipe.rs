use std::time::Duration;

use rx_core_common::{Observable, Operator, Scheduler, SchedulerHandle, Signal};

use crate::operator::SubscribeOnOperator;

pub trait ObservablePipeExtensionSubscribeOn<'o, T, S>:
	'o + Observable<Out = T> + Sized + Send + Sync
where
	'o: 'static,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	/// # [SubscribeOnOperator]
	///
	/// The `subscribe_on` operator schedules the subscription to the upstream
	/// observable on the provided scheduler.
	///
	/// This only affects **when** the upstream subscription starts. It does not
	/// alter when upstream `next`, `error`, or `complete` signals are emitted.
	///
	/// `subscribe_on` uses a `0` delay by default, matching RxJS. Use
	/// `subscribe_on_with_delay` to customize the delay.
	#[inline]
	fn subscribe_on_with_delay(
		self,
		delay: Duration,
		scheduler: SchedulerHandle<S>,
	) -> <SubscribeOnOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		SubscribeOnOperator::new(delay, scheduler).operate(self)
	}

	/// # [SubscribeOnOperator]
	///
	/// The `subscribe_on` operator schedules the subscription to the upstream
	/// observable on the provided scheduler.
	///
	/// This only affects **when** the upstream subscription starts. It does not
	/// alter when upstream `next`, `error`, or `complete` signals are emitted.
	#[inline]
	fn subscribe_on(
		self,
		scheduler: SchedulerHandle<S>,
	) -> <SubscribeOnOperator<T, Self::OutError, S> as Operator<'o>>::OutObservable<Self> {
		self.subscribe_on_with_delay(Duration::ZERO, scheduler)
	}
}

impl<'o, O, T, S> ObservablePipeExtensionSubscribeOn<'o, T, S> for O
where
	O: 'o + Observable<Out = T> + Send + Sync,
	T: Signal,
	S: 'static + Scheduler + Send + Sync,
	'o: 'static,
{
}
