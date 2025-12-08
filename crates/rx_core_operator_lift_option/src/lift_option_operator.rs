use core::marker::PhantomData;

use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Never, Operator, Signal, Subscriber};

use crate::LiftOptionSubscriber;

#[derive(RxOperator)]
#[rx_in(Option<In>)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct LiftOptionOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> Default for LiftOptionOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn default() -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for LiftOptionOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= LiftOptionSubscriber<Destination>
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
		LiftOptionSubscriber::new(destination)
	}
}

impl<In, InError> Clone for LiftOptionOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	fn clone(&self) -> Self {
		Self {
			_phantom_data: PhantomData,
		}
	}
}
