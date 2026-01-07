use core::marker::PhantomData;
use std::num::NonZero;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Never, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::BufferCountSubscriber;

#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Vec<In>)]
#[rx_out_error(InError)]
pub struct BufferCountOperator<In, InError = Never>
where
	In: Signal,
	InError: Signal,
{
	buffer_size: NonZero<usize>,
	_phantom_data: PhantomData<(In, InError)>,
}

impl<In, InError> BufferCountOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	pub fn new(buffer_size: usize) -> Self {
		Self {
			buffer_size: NonZero::new(buffer_size).unwrap_or(NonZero::<usize>::MIN),
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError> ComposableOperator for BufferCountOperator<In, InError>
where
	In: Signal,
	InError: Signal,
{
	type Subscriber<Destination>
		= BufferCountSubscriber<In, Destination>
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
		BufferCountSubscriber::new(destination, self.buffer_size)
	}
}
