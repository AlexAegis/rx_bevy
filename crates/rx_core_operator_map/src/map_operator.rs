use core::marker::PhantomData;

use derive_where::derive_where;
use rx_core_macro_operator_derive::RxOperator;
use rx_core_traits::{Operator, Signal, Subscriber};

use crate::MapSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
#[derive(RxOperator)]
#[rx_in(In)]
#[rx_in_error(InError)]
#[rx_out(Out)]
#[rx_out_error(InError)]
pub struct MapOperator<In, InError, Mapper, Out = In>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: Signal,
{
	mapper: Mapper,
	_phantom_data: PhantomData<(In, InError, Out)>,
}

impl<In, InError, Mapper, Out> MapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: Signal,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out> Operator for MapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: Signal,
{
	type Subscriber<Destination>
		= MapSubscriber<In, InError, Mapper, Out, Destination>
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
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, Out> Clone for MapOperator<In, InError, Mapper, Out>
where
	In: Signal,
	InError: Signal,
	Mapper: 'static + Fn(In) -> Out + Clone + Send + Sync,
	Out: Signal,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
