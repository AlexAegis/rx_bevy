use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{ComposableOperator, Never, Signal, Subscriber};

use crate::internal::OnNextSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator, Clone)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(InError)]
pub struct OnNextOperator<OnNext, In, InError = Never>
where
	OnNext: 'static
		+ FnMut(&In, &mut dyn Subscriber<In = In, InError = InError>) -> bool
		+ Send
		+ Sync
		+ Clone,
	In: Signal,
	InError: Signal,
{
	on_next: OnNext,
	_phantom_data: PhantomData<fn(In, InError) -> (In, InError)>,
}

impl<OnNext, In, InError> OnNextOperator<OnNext, In, InError>
where
	OnNext: 'static
		+ FnMut(&In, &mut dyn Subscriber<In = In, InError = InError>) -> bool
		+ Send
		+ Sync
		+ Clone,
	In: Signal,
	InError: Signal,
{
	pub fn new(on_next: OnNext) -> Self {
		Self {
			on_next,
			_phantom_data: PhantomData,
		}
	}
}

impl<OnNext, In, InError> ComposableOperator for OnNextOperator<OnNext, In, InError>
where
	OnNext: 'static
		+ FnMut(&In, &mut dyn Subscriber<In = In, InError = InError>) -> bool
		+ Send
		+ Sync
		+ Clone,
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= OnNextSubscriber<OnNext, Destination>
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
		OnNextSubscriber::new(destination, self.on_next.clone())
	}
}
