use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Observable, Operator, Signal, Subscriber};

use crate::ExhaustAllSubscriber;

#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In::Out)]
#[rx_out_error(In::OutError)]
pub struct ExhaustAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for ExhaustAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for ExhaustAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	type Subscriber<Destination>
		= ExhaustAllSubscriber<In, InError, Destination>
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
		ExhaustAllSubscriber::new(destination)
	}
}

impl<In, InError> Clone for ExhaustAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
