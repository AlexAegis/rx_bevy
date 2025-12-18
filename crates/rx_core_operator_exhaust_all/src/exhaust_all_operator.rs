use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_subscriber_exhaust::ExhaustSubscriberProvider;
use rx_core_subscriber_higher_order_all::HigherOrderAllSubscriber;
use rx_core_traits::{ComposableOperator, Observable, Signal, Subscriber};

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

impl<In, InError> ComposableOperator for ExhaustAllOperator<In, InError>
where
	In: Observable + Signal,
	InError: Signal + Into<In::OutError>,
{
	type Subscriber<Destination>
		= HigherOrderAllSubscriber<In, InError, ExhaustSubscriberProvider, Destination>
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
		HigherOrderAllSubscriber::new(destination, 1)
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
