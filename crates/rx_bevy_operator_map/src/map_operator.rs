use std::marker::PhantomData;

use derive_where::derive_where;
use rx_bevy_core::{ObservableOutput, ObserverInput, Operator, SignalContext, Subscriber};

use crate::MapSubscriber;

#[derive_where(Debug)]
#[derive_where(skip_inner(Debug))]
pub struct MapOperator<In, InError, Mapper, Out = In, Context = ()>
where
	Mapper: Fn(In) -> Out,
{
	pub mapper: Mapper,
	pub _phantom_data: PhantomData<(In, InError, Out, Context)>,
}

impl<In, InError, Mapper, Out, Context> MapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Out,
{
	pub fn new(mapper: Mapper) -> Self {
		Self {
			mapper,
			_phantom_data: PhantomData,
		}
	}
}

impl<In, InError, Mapper, Out, Context> Operator for MapOperator<In, InError, Mapper, Out, Context>
where
	In: 'static,
	InError: 'static,
	Mapper: 'static + Clone + Fn(In) -> Out,
	Out: 'static,
	Context: SignalContext,
{
	type Context = Context;
	type Subscriber<Destination>
		= MapSubscriber<In, InError, Mapper, Out, Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>;

	#[inline]
	fn operator_subscribe<Destination>(
		&mut self,
		destination: Destination,
		_context: &mut Self::Context,
	) -> Self::Subscriber<Destination>
	where
		Destination:
			'static + Subscriber<In = Self::Out, InError = Self::OutError, Context = Self::Context>,
	{
		MapSubscriber::new(destination, self.mapper.clone())
	}
}

impl<In, InError, Mapper, Out, Context> ObservableOutput
	for MapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Out,
	Out: 'static,
	InError: 'static,
{
	type Out = Out;
	type OutError = InError;
}

impl<In, InError, Mapper, Out, Context> ObserverInput
	for MapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Fn(In) -> Out,
	In: 'static,
	InError: 'static,
{
	type In = In;
	type InError = InError;
}

impl<In, InError, Mapper, Out, Context> Clone for MapOperator<In, InError, Mapper, Out, Context>
where
	Mapper: Clone + Fn(In) -> Out,
{
	fn clone(&self) -> Self {
		Self {
			mapper: self.mapper.clone(),
			_phantom_data: PhantomData,
		}
	}
}
