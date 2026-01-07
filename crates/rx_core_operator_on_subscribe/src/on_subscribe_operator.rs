use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct OnSubscribeOperator<OnSubscribe, In, InError = Never>
where
	OnSubscribe: 'static + FnMut(&mut dyn Subscriber<In = In, InError = InError>) + Send + Sync,
	In: Signal,
	InError: Signal,
{
	on_subscribe: OnSubscribe,
	_phantom_data: PhantomData<fn(In, InError) -> (In, InError)>,
}

impl<OnSubscribe, In, InError> OnSubscribeOperator<OnSubscribe, In, InError>
where
	OnSubscribe: 'static + FnMut(&mut dyn Subscriber<In = In, InError = InError>) + Send + Sync,
	In: Signal,
	InError: Signal,
{
	pub fn new(on_subscribe: OnSubscribe) -> Self {
		Self {
			on_subscribe,
			_phantom_data: PhantomData,
		}
	}
}

impl<OnSubscribe, In, InError> ComposableOperator for OnSubscribeOperator<OnSubscribe, In, InError>
where
	OnSubscribe: 'static + FnMut(&mut dyn Subscriber<In = In, InError = InError>) + Send + Sync,
	In: Signal,
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
		(self.on_subscribe)(&mut destination);
		destination
	}
}
