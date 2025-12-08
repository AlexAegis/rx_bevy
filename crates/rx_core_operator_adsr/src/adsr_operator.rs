use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber};

use crate::{AdsrSignal, AdsrSubscriber, AdsrTrigger, operator::AdsrOperatorOptions};

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(RxOperator)]
#[rx_in(AdsrTrigger)]
#[rx_in_error(InError)]
#[rx_out(AdsrSignal)]
#[rx_out_error(InError)]
pub struct AdsrOperator<InError>
where
	InError: Signal,
{
	options: AdsrOperatorOptions,
	_phantom_data: PhantomData<InError>,
}

impl<InError> AdsrOperator<InError>
where
	InError: Signal,
{
	pub fn new(options: AdsrOperatorOptions) -> Self {
		Self {
			options,
			_phantom_data: PhantomData,
		}
	}
}

impl<InError> Operator for AdsrOperator<InError>
where
	InError: Signal,
{
	type Subscriber<Destination>
		= AdsrSubscriber<InError, Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		AdsrSubscriber::new(destination, self.options.clone())
	}
}
