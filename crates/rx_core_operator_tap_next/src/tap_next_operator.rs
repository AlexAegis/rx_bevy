use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::TapNextSubscriber;

#[derive_where(Debug, Clone)]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct TapNextOperator<In, InError, OnNext>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(&In) + Clone + Send + Sync,
{
	#[derive_where(skip(Debug))]
	on_next: OnNext,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError, OnNext> TapNextOperator<In, InError, OnNext>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(&In) + Clone + Send + Sync,
{
	pub fn new(on_next: OnNext) -> Self {
		Self {
			on_next,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, OnNext> ComposableOperator for TapNextOperator<In, InError, OnNext>
where
	In: Signal,
	InError: Signal,
	OnNext: 'static + FnMut(&In) + Clone + Send + Sync,
{
	type Subscriber<Destination>
		= TapNextSubscriber<In, InError, OnNext, Destination>
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
		TapNextSubscriber::new(destination, self.on_next.clone())
	}
}
