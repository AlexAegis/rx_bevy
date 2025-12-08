use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Scheduler, Signal, Subscriber};

use crate::{DelaySubscriber, operator::DelayOperatorOptions};

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
	options: DelayOperatorOptions<S>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, S> DelayOperator<In, InError, S>
where
	In: Signal,
	InError: Signal,
	S: Scheduler,
{
	pub fn new(options: DelayOperatorOptions<S>) -> Self {
		Self {
			options,
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
		DelaySubscriber::new(destination, self.options.clone())
	}
}
