use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Scheduler, SchedulerHandle, Signal, Subscriber};

use crate::{AdsrSignal, AdsrSubscriber, AdsrTrigger, operator::AdsrOperatorOptions};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(RxOperator)]
#[rx_in(AdsrTrigger)]
#[rx_in_error(InError)]
#[rx_out(AdsrSignal)]
#[rx_out_error(InError)]
pub struct AdsrOperator<InError, S>
where
	InError: Signal,
	S: Scheduler,
{
	options: AdsrOperatorOptions,
	scheduler: SchedulerHandle<S>,
	_phantom_data: PhantomData<InError>,
}

impl<InError, S> AdsrOperator<InError, S>
where
	InError: Signal,
	S: Scheduler,
{
	pub fn new(options: AdsrOperatorOptions, scheduler: SchedulerHandle<S>) -> Self {
		Self {
			options,
			scheduler,
			_phantom_data: PhantomData,
		}
	}
}

impl<InError, S> Operator for AdsrOperator<InError, S>
where
	InError: Signal,
	S: 'static + Scheduler,
{
	type Subscriber<Destination>
		= AdsrSubscriber<InError, Destination, S>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		AdsrSubscriber::new(destination, self.options.clone(), self.scheduler.clone())
	}
}
