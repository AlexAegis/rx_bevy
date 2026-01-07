use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_common::{ComposableOperator, Signal, Subscriber};
use rx_core_macro_operator_derive::RxOperator;

use crate::MapErrorSubscriber;

#[derive_where(Debug, Clone)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(In)]
#[rx_out_error(OutError)]
pub struct MapErrorOperator<In, InError, ErrorMapper, OutError = InError>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> OutError + Clone + Send + Sync,
	OutError: Signal,
{
	error_mapper: ErrorMapper,
	_phantom_data: PhantomData<fn(In, InError, OutError) -> (In, InError, OutError)>,
}

impl<In, InError, ErrorMapper, OutError> MapErrorOperator<In, InError, ErrorMapper, OutError>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> OutError + Clone + Send + Sync,
	OutError: Signal,
{
	pub fn new(error_mapper: ErrorMapper) -> Self {
		Self {
			error_mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, ErrorMapper, OutError> ComposableOperator
	for MapErrorOperator<In, InError, ErrorMapper, OutError>
where
	In: Signal,
	InError: Signal,
	ErrorMapper: 'static + FnOnce(InError) -> OutError + Clone + Send + Sync,
	OutError: Signal,
{
	type Subscriber<Destination>
		= MapErrorSubscriber<In, InError, ErrorMapper, OutError, Destination>
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
		MapErrorSubscriber::new(destination, self.error_mapper.clone())
	}
}
