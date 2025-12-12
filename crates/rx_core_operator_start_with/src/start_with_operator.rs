use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber};

#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct StartWithOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal,
{
	start_with: In,
	_phantom_data: PhantomData<InError>,
}

impl<In, InError> StartWithOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal,
{
	pub fn new(start_with: In) -> Self {
		Self {
			start_with,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> Operator for StartWithOperator<In, InError>
where
	In: Signal + Clone,
	InError: Signal,
{
	type Subscriber<Destination>
		= Destination
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		mut destination: Destination,
	) -> Self::Subscriber<Destination>
	where
		Destination: 'static + Subscriber<In = Self::Out, InError = Self::OutError> + Send + Sync,
	{
		destination.next(self.start_with.clone());
		destination
	}
}
