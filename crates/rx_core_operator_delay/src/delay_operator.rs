use core::marker::PhantomData;
use std::time::Duration;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle, Signal, Subscriber};

use crate::DelaySubscriber;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct DelayOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	duration: Duration,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, S> DelayOperator<In, InError, S>
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

impl<In, InError, S> Operator for DelayOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: 'static + Scheduler + Send + Sync,
{
	type Subscriber<Destination>
		= DelaySubscriber<Destination, S>
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
		DelaySubscriber::new(destination, self.duration, self.scheduler.clone())
	}
}
