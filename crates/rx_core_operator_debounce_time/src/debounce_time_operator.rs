use core::marker::PhantomData;
use std::time::Duration;

use rx_core_common::{
	ComposableOperator, PhantomInvariant, Scheduler, SchedulerHandle, Signal, Subscriber,
};
use rx_core_macro_operator_derive::RxOperator;

use crate::DebounceTimeSubscriber;

/// # [DebounceTimeOperator]
///
/// The `debounce_time` operator emits the most recent upstream value only after
/// the specified duration passes without another emission.
///
/// Upstream completion and cancellation can happen instantly if there are no
/// pending debounced values, otherwise it will complete or cancel once the
/// pending debounced value has been emitted.
///
/// Upstream errors are immediately propagated downstream, cancelling any
/// pending debounced value.
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct DebounceTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	duration: Duration,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError, S> DebounceTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	pub fn new(duration: Duration, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			duration,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, S> ComposableOperator for DebounceTimeOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscriber<Destination>
		= DebounceTimeSubscriber<Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		DebounceTimeSubscriber::new(destination, self.duration, self.scheduler.clone())
	}
}
