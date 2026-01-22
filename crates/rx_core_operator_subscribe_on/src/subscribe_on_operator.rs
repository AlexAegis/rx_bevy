use core::marker::PhantomData;
use std::time::Duration;

use rx_core_common::{Observable, Operator, PhantomInvariant, Scheduler, SchedulerHandle, Signal};
use rx_core_macro_operator_derive::RxOperator;

use crate::observable::SubscribeOnObservable;

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
#[derive(RxOperator, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct SubscribeOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	delay: Duration,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, S> SubscribeOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	pub fn new(delay: Duration, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			delay,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<'o, In, InError, S> Operator<'o> for SubscribeOnOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: 'static + Scheduler + Send + Sync,
	'o: 'static,
{
	type OutObservable<InObservable>
		= SubscribeOnObservable<InObservable, S>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync;

	#[inline]
	fn operate<InObservable>(self, source: InObservable) -> Self::OutObservable<InObservable>
	where
		InObservable: 'o + Observable<Out = Self::In, OutError = Self::InError> + Send + Sync,
	{
		SubscribeOnObservable::new(source, self.delay, self.scheduler)
	}
}
