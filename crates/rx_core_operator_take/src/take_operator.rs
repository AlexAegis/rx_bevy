use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, PhantomInvariant, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::TakeSubscriber;

#[derive_where(Debug, Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct TakeOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	count: usize,
	_phantom_data: PhantomInvariant<(In, InError)>,
}

impl<In, InError> TakeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(count: usize) -> Self {
		Self {
			count,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for TakeOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= TakeSubscriber<Destination>
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
		TakeSubscriber::new(destination, self.count)
	}
}
